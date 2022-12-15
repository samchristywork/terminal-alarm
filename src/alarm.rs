use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create(name: &str, path: &str) {
    let filename = path.to_string()
        + format!(
            "{:?}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        )
        .as_str();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    file.write(name.as_bytes()).unwrap();
    file.write("\n".as_bytes()).unwrap();
}
