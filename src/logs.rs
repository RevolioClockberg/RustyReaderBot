use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::*;


// Write logs on local logs/logs.txt file
pub fn write_logs(logs: String) {
    let file_path = "/var/www/RustyReaderBot/files/logs.txt".to_owned();
    let mut data_file = OpenOptions::new().append(true).open(file_path).expect("Can't open log file !");
    let date = Local::now().format("%Y/%m/%d-%H:%M").to_string();
    let log = format!("{} - {}\n", date, logs);
    data_file.write_all(log.as_bytes()).expect("write failed");
}
