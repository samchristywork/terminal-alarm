use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
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

pub fn list(path: &str) {
    for file in std::fs::read_dir(path).unwrap() {
        let filename = file.unwrap().file_name().to_str().unwrap().to_string();

        let mut file = File::open(path.to_string() + filename.as_str()).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.split("\n") {
            if line != "" {
                println!("{} {}", filename, line);
            }
        }
    }
}
