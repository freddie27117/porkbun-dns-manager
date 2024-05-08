use crate::deblogger::deblogger;
use crate::structs::JSONdata;
use crate::utils::FILE_LOCATION;
use home::home_dir;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn install() {
    make_directory();
    make_log();
    make_default_json();
}

fn make_directory() {
    let home_dir = home_dir().expect("Could not locate the home directory");
    let directory = home_dir.join(Path::new(FILE_LOCATION));

    fs::create_dir_all(directory).expect("Could not create folders");
}

fn make_log() {
    deblogger("Created directory");
    deblogger("Initialized log file");
}

fn make_default_json() {
    let home_dir: std::path::PathBuf = home_dir().expect("Could not locate home directory");
    let file_location = home_dir.join(Path::new(FILE_LOCATION)).join("config.json");

    let data = JSONdata {
        domain: "DOMAIN-HERE".to_string(),
        subdomain: "SUBDOMAIN-HERE".to_string(),
        ttl: "600".to_string(),
        secretapikey: "SECRET-KEY".to_string(),
        apikey: "API-KEY".to_string(),
    };

    let json_data =
        serde_json::to_string_pretty(&data).expect("Could not format the default json data");

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(&file_location)
        .expect("err");

    write!(file, "{}", json_data).expect("Error");
    deblogger("Created JSON file");
    deblogger("DONE!");
    println!(
        "REMINDER: Be sure to update {} with your dns info",
        &file_location.to_string_lossy()
    );
}
