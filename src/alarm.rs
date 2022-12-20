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

pub fn list(path: &str) {
    let mut items: Vec<String> = Vec::new();

    for file in std::fs::read_dir(path).unwrap() {
        let filename = file.unwrap().file_name().to_str().unwrap().to_string();

        let mut file = File::open(path.to_string() + filename.as_str()).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.split("\n") {
            if line != "" {
                if filename.parse::<u64>().unwrap()
                    > SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                {
                    items.push(format!("{} {}", filename, line));
                } else {
                    items.push(format!("{} {} (overdue)", filename, line));
                }
            }
        }
    }

    items.sort();
    for item in items {
        println!("{}", item);
    }
}

pub fn monitor() {}
