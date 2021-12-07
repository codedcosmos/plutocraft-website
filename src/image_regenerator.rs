use std::fs::OpenOptions;
use std::process::Command;
use std::thread;

pub fn regenerate_image() {
    thread::spawn(|| {
        info!("Regenerating image!");
        let output = Command::new("anvil")
            .arg("render")
            .arg("pretend/world-2021-12-06/world/")
            .arg("--palette")
            .arg("working/palette.tar.gz")
            .output()
            .expect("Failed to build new world image");

        if !output.status.success() {
            if let Ok(str) = std::str::from_utf8(&output.stdout.as_slice()) {
                info!("{}", str);
            }
        }

        println!("{:?}", output);
    });
}