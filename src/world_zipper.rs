use std::fs::OpenOptions;
use std::process::Command;
use std::{fs, thread, time};

pub fn rezip_world() {
    thread::spawn(|| {
        info!("Sleeping before backing up world");
        thread::sleep(time::Duration::from_millis(100));

        info!("Regenerating image!");
        let zip_name = format!("backups/world-{}.zip", chrono::offset::Local::now().format("%Y-%m-%d-%s"));

        let output = Command::new("zip")
            .arg("-r")
            .arg(zip_name.clone())
            .arg("pretend/world-2021-12-06/world/")
            .output()
            .expect("Failed to build new world image");

        if !output.status.success() {
            if let Ok(str) = std::str::from_utf8(&output.stdout.as_slice()) {
                info!("{}", str);
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
                            println!("Deleting {:?}", path);
                            fs::remove_file(path);
                        }
                    }
                }
            }
        }
    });
}