use crate::structs::JSONdata;
use home::home_dir;
use std::fs::File;
use std::io::BufReader;

pub const FILE_LOCATION: &str = ".config/porkbun-manager";

pub fn get_json_data() -> JSONdata {
    let home_path = home_dir().expect("Unable to fetch your home directory");
    let file_path = home_path.join(FILE_LOCATION).join("dns-info.json");

    let file = File::open(&file_path).expect("Could not open the config file");
    let reader = BufReader::new(&file);
    let contents: JSONdata =
        serde_json::from_reader(reader).expect("Unable to serialize the json data");

    contents
}
