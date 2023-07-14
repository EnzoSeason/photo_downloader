use std::io;

use chrono::{FixedOffset, Utc};

pub fn get_source() -> String {
    String::from(r"C:\Users\wlp69\Downloads")
}

pub fn get_destination() -> String {
    let mut destination = String::from(r"D:\记忆\照片\");

    destination.push_str(&read_destination());

    destination
}

fn read_destination() -> String {
    println!("备份到哪里？（例如：2023年旅游 6，7月宜兴，广州）");

    let mut destination = String::new();

    io::stdin()
        .read_line(&mut destination)
        .expect("Failed to read line");

    destination.trim().replace(r" ", r"\")
}

fn get_default_destination() -> String {
    let now = Utc::now();
    let china_timezone =
        { FixedOffset::east_opt(8 * 3600).expect("FixedOffset::east out of bounds") };

    let now = now.with_timezone(&china_timezone);
    let now = format!("{}", now.format("%Y-%m-%d-%H-%M-%S"));

    now
}
