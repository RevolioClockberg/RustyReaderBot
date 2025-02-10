use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::*;


// Write logs on local logs/logs.txt file
pub fn write_logs(logs: String) {
    let file_path = format!("{:?}/errors.log", env::var("RUSTY_BOT_LOGS"));
    let mut data_file = OpenOptions::new().append(true).open(file_path).expect("Can't open log file !");
    let date = Local::now().format("%Y/%m/%d-%H:%M").to_string();
    let log = format!("{} - {}\n", date, logs);
    data_file.write_all(log.as_bytes()).expect("write logs failed");
    if Ok(String::from("TRUE")) == env::var("RUSTY_DEBUG") {
        write_debug(logs);
    }
}

pub fn write_debug(logs: String) {
    let file_path = format!("{:?}/debug.log", env::var("RUSTY_BOT_LOGS"));
    let mut data_file = OpenOptions::new().append(true).open(file_path).expect("Can't open debug file !");
    let date = Local::now().format("%Y/%m/%d-%H:%M").to_string();
    let log = format!("{} - {}\n", date, logs);
    data_file.write_all(log.as_bytes()).expect("write logs failed");
}