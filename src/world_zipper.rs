use std::fs::OpenOptions;
use std::process::Command;
use std::{fs, thread, time};
use crate::log;

pub fn rezip_world() {
    thread::spawn(|| {
        log!("Sleeping before backing up world");
        thread::sleep(time::Duration::from_millis(100));

        log!("Regenerating world map!");
        let zip_name = format!("backups/world-{}.zip", chrono::offset::Local::now().format("%Y-%m-%d-%s"));
        let world_path = include_str!("../worldpath.txt").trim();

        let output = Command::new("zip")
            .arg("-r")
            .arg(zip_name.clone())
            .arg(world_path)
            .output()
            .expect("Failed to build new world image");

        if !output.status.success() {
            if let Ok(str) = std::str::from_utf8(&output.stdout.as_slice()) {
                log!("Zip log output: {}", str);
            }
            return;
        }

        // Delete other zips
        if let Ok(directory) = fs::read_dir("backups") {
            for entry in directory {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(string) = path.to_str() {
                        if !string.ends_with(&zip_name) {
                            log!("Deleting {:?}", path);
                            fs::remove_file(path);
                        }
                    }
                }
            }
        }
    });
}