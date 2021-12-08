#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);

        println!("{}", res);

        use std::io::Write;
        let res = format!("{}\n", res);

        let file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("out.log");

        if let Ok(mut file) = file {
            file.write_all(res.as_bytes());
        }
    }}
}