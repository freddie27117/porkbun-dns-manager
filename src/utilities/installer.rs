use crate::utilities::deblogger::{deblogger, deblogger_fatal};
use crate::utilities::structs::JSONdata;
use crate::utilities::utils::FILE_LOCATION;
use home::home_dir;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn install() {
    make_directory();
    make_default_json();
}

fn make_directory() {
    let home_dir = home_dir().expect("Unable to fetch your home directory");
    let directory = home_dir.join(Path::new(FILE_LOCATION));

    fs::create_dir_all(directory).expect("Could not create working directory");
}

fn make_default_json() {
    let home_dir: std::path::PathBuf = home_dir().expect("Unable to fetch your home directory");
    let file_location = home_dir.join(Path::new(FILE_LOCATION)).join("config.json");

    let data = get_install_info();

    let json_data = serde_json::to_string_pretty(&data)
        .unwrap_or_else(|error| deblogger_fatal("Unable to format json credentials", error));

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_location)
        .expect("err");

    write!(file, "{}", json_data)
        .unwrap_or_else(|error| deblogger_fatal("Unable to write credentials to json file", error));
    deblogger("Created JSON file");
}

fn get_install_info() -> JSONdata {
    let prompts = ["Domain", "Subdomain", "ttl", "Secret API Key", "API Key"];
    let mut responses: Vec<String> = Vec::new();

    for prompt in prompts.iter() {
        let mut line: String = String::new();
        print!("Enter {}: ", prompt);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();

        responses.push(line.trim().to_string());
    }

    return JSONdata {
        domain: responses[0].to_string(),
        subdomain: responses[1].to_string(),
        ttl: responses[2].to_string(),
        secretapikey: responses[3].to_string(),
        apikey: responses[4].to_string(),
    };
}
