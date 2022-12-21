use soloud::*;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

pub struct ListItem {
    time: u64,
    label: String,
    flag: bool,
}

impl ListItem {
    pub fn new(time: u64, label: String) -> Self {
        let mut n = Self {
            time,
            label,
            flag: false,
        };

        n.flag = n.is_overdue();

        n
    }
    fn is_overdue(&self) -> bool {
        self.time
            > SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
    }
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_overdue() {
            write!(f, "{} {}", self.time, self.label)
        } else {
            write!(f, "{} {} (overdue)", self.time, self.label)
        }
    }
}

pub fn consume() {}

pub fn create(name: &str, path: &str, time: u64) {
    let filename = path.to_string() + format!("{:?}", time).as_str();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    file.write(name.as_bytes()).unwrap();
    file.write("\n".as_bytes()).unwrap();
}

pub fn list(path: &str) -> Vec<ListItem> {
    let mut items = Vec::new();

    for file in std::fs::read_dir(path).unwrap() {
        let filename = file.unwrap().file_name().to_str().unwrap().to_string();

        let mut file = File::open(path.to_string() + filename.as_str()).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.split("\n") {
            if line != "" {
                let time = filename.parse::<u64>().unwrap();
                items.push(ListItem::new(time, line.to_string()));
            }
        }
    }

    items.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    items
}

pub fn monitor(path: &str) {
    let mut items = list(path);

    for item in items.iter_mut() {
        println!("{}", item);
    }

    loop {
        for mut item in items.iter_mut() {
            if item.flag != item.is_overdue() {
                item.flag = item.is_overdue();
                println!("{}", item);
                let sl = Soloud::default().unwrap();

                let mut wav = audio::Wav::default();

                wav.load(&std::path::Path::new("./triangle.wav")).unwrap();

                sl.play(&wav);
                while sl.voice_count() > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
