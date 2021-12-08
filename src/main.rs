mod image_regenerator;
mod world_zipper;
mod log;
mod random_msg;

#[macro_use]
extern crate rocket;
extern crate lazy_static;

use std::alloc::System;
use std::borrow::BorrowMut;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::ops::Deref;
use std::path::Path;
use lazy_static::lazy_static;
use rocket::{Build, Request, Rocket};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::content;
use rocket::response::content::{Css, Html};
use std::sync::Mutex;
use std::time::SystemTime;
use rocket::error::ErrorKind::Config;
use crate::image_regenerator::regenerate_image;

lazy_static! {
    static ref IMAGE_LOCK: Mutex<(SystemTime, bool)> = Mutex::new((SystemTime::now(), false));
    static ref WORLD_LOCK: Mutex<(SystemTime, bool)> = Mutex::new((SystemTime::now(), false));
}

const IMAGE_LOCK_DURATION_TEXT: &str = "30 minutes";
const WORLD_LOCK_DURATION_TEXT: &str = "~7 days";

const IMAGE_LOCK_DURATION: u64 = 60*30;
const WORLD_LOCK_DURATION: u64 = 60*23*7;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    if req.uri().to_string().ends_with("world.zip") {
        format!("World zip not available just yet, try again in a few minutes")
    } else if req.uri().to_string().ends_with("world-map.png") {
        format!("World image not available just yet, try again in a few minutes")
    } else {
        format!("404 '{}' not found!", req.uri())
    }
}

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

#[get("/world-map.png")]
pub async fn world_map() -> Option<NamedFile> {
    log!("Requested world map");
    if let Ok(mut image_lock) = IMAGE_LOCK.lock() {
        // See if n time has passed
        let (mut time, mut unlocked) = *image_lock;

        if let Ok(duration) = time.elapsed() {
            if duration.as_secs() >= IMAGE_LOCK_DURATION || !unlocked {
                time = SystemTime::now();
                unlocked = true;
                regenerate_image();
            }
        }
    }
    // Grab older
    let mut path = Path::new("map.png");

    NamedFile::open(path).await.ok()
}

#[get("/world.zip")]
pub async fn world_download() -> Option<NamedFile> {
    log!("Requested world download");
    if let Ok(mut world_lock) = WORLD_LOCK.lock() {
        // See if n time has passed
        let (mut time, mut unlocked) = *world_lock;

        if let Ok(duration) = time.elapsed() {
            if duration.as_secs() >= IMAGE_LOCK_DURATION || !unlocked {
                time = SystemTime::now();
                unlocked = true;
                world_zipper::rezip_world();
            }
        }
    }
    // Find oldest zip
    let mut oldest_zip = None;

    if let Ok(directory) = fs::read_dir("backups") {
        for entry in directory {
            if let Ok(entry) = entry {
                let path = entry.path();

                if let Ok(metadata) = fs::metadata(&path) {
                    if metadata.is_file() {
                        if let Ok(metadata) = metadata.modified() {
                            if let Ok(elapsed) = metadata.elapsed() {
                                if let Some((duration, _)) = oldest_zip {
                                    if elapsed.as_secs() > duration {
                                        oldest_zip = Some((elapsed.as_secs(), path));
                                    }
                                } else {
                                    oldest_zip = Some((elapsed.as_secs(), path));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some((_, file)) = oldest_zip {
        NamedFile::open(file).await.ok()
    } else {
        None
    }
}

#[get("/logo.png")]
pub async fn logo_png() -> Option<NamedFile> {
    // Grab older
    let mut path = Path::new("res/logo-boxed.png");
    NamedFile::open(path).await.ok()
}

#[get("/")]
fn index() -> Html<String> {
    let html = include_str!("../web/home.html");

    let html = html.replace("%worldupdate%", WORLD_LOCK_DURATION_TEXT);
    let html = html.replace("%mapupdate%", IMAGE_LOCK_DURATION_TEXT);
    let html = html.replace("%randommessage%", random_msg::get_random_message().as_str());

    content::Html(html)
}

#[get("/web.css")]
fn web_css() -> Css<&'static str> {
    content::Css(include_str!("../web/web.css"))
}

#[launch]
fn rocket() -> Rocket<Build> {
    let mut config = rocket::Config::release_default();
    config.cli_colors = false;
    config.address = IpAddr::V4(Ipv4Addr::new(0,0,0,0));

    log!("Launching website");

    rocket::build().configure(config)
        .mount("/", routes![world_map, world_download, logo_png, index, web_css])
        .register("/", catchers![internal_error, not_found, default])
}
