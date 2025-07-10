use crate::utilities;
use crate::utilities::deblogger::deblogger_fatal;
use crate::utilities::structs::JSONdata;
use home::home_dir;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub const FILE_LOCATION: &str = ".config/porkbun-manager";

pub fn get_json_data() -> JSONdata {
    let home_path = home_dir().expect("Unable to fetch your home directory");
    let file_path = home_path.join(FILE_LOCATION).join("config.json");

    if file_path.exists() {
        return retrieve_data(file_path);
    } else {
        utilities::install();
        return retrieve_data(file_path);
    }
}

fn retrieve_data(file_path: PathBuf) -> JSONdata {
    let file = File::open(&file_path)
        .unwrap_or_else(|e| deblogger_fatal("Unable to open the config.json file", e));
    let reader = BufReader::new(&file);
    let contents: JSONdata = serde_json::from_reader(reader)
        .unwrap_or_else(|e| deblogger_fatal("Unable to serialize the retrieved json data", e));

    return contents;
}
