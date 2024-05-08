use crate::structs::JSONdata;
use home::home_dir;
use std::fs::File;
use std::io::BufReader;
// use std::path::Path;

pub fn get_json_data() -> JSONdata {
    let home_path = home_dir().expect("Unable to fetch your home directory");
    let file_path = home_path.join(".config/porkbun-dns-buddy/dns-info.json");

    let file = File::open(&file_path).expect("Could not open the config file");
    let reader = BufReader::new(&file);
    let contents: JSONdata =
        serde_json::from_reader(reader).expect("Unable to serialize the json data");

    contents
}
