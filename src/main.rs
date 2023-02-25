use std::env::var;
use std::fs::create_dir_all;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod alarm;
pub mod audio;
pub mod wav;

fn main() {
    let foo = var("HOME").unwrap() + "/.config/terminal-alarm/";
    create_dir_all(foo.clone()).unwrap();
    audio::generate_triangle_wave();

    alarm::create(
        "Alarm in 5 seconds",
        foo.as_str(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 5,
    );

    //let items = alarm::list(foo.as_str());

    //for item in items {
    //    println!("{}", item);
    //}

    alarm::monitor(foo.as_str());
}
