use std::fs::OpenOptions;
use std::process::Command;
use std::thread;
use crate::log;

pub fn regenerate_image() {
    thread::spawn(|| {
        log!("Regenerating image!");

        let world_path = include_str!("../worldpath.txt").trim();

        let output = Command::new("anvil")
            .arg("render")
            .arg(world_path)
            .arg("--palette")
            .arg("working/palette.tar.gz")
            .output()
            .expect("Failed to build new world image");

        if !output.status.success() {
            if let Ok(str) = std::str::from_utf8(&output.stdout.as_slice()) {
                log!("anvil render {} --palette working/palette.tar.gz", world_path);
                log!("Zip log output: {}", str);
            }
        } else {
            log!("Generated image");
        }
    });
}