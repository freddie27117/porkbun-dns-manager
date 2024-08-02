use crate::deblogger::deblogger_fatal;
use crate::structs::JSONdata;
use home::home_dir;
use std::fs::File;
use std::io::BufReader;

pub const FILE_LOCATION: &str = ".config/porkbun-manager";

pub fn get_json_data() -> JSONdata {
    let home_path = home_dir().expect("Unable to fetch your home directory");
    let file_path = home_path.join(FILE_LOCATION).join("config.json");

    let file = File::open(&file_path)
        .unwrap_or_else(|e| deblogger_fatal("Unable to open the config.json file", e.to_string()));
    let reader = BufReader::new(&file);
    let contents: JSONdata = serde_json::from_reader(reader).unwrap_or_else(|e| {
        deblogger_fatal("Unable to serialize the retrieved json data", e.to_string())
    });

    contents
}
