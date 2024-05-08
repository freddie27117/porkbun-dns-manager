use crate::utils;
use chrono::Local;
use home::home_dir;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn deblogger(message: impl AsRef<str>) {
    let time = Local::now().format("%H:%M:%S");
    let message = message.as_ref();
    let formated_message = format!("[{}] {}", time, message);
    println!("{}", formated_message);
    log(formated_message)
}

pub fn deblogger_fatal(message: impl AsRef<str>, error: String) -> ! {
    let time = Local::now().format("%H:%M:%S");
    let message = message.as_ref();
    let formated_message = format!("[{}] FATAL: {} -> {}", time, message, error);
    println!("{}", formated_message);
    log(formated_message);
    std::process::exit(1);
}

fn log(data: String) {
    let home_dir = home_dir().unwrap();
    // let file_location = home_dir.join(Path::new(".config/porkbun-dns-buddy/log"));
    let file_location = home_dir.join(Path::new(utils::FILE_LOCATION)).join("log");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_location)
        .expect("Error");

    writeln!(file, "{}", data).expect("Error")
}
