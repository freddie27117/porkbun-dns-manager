use crate::deblogger::deblogger_fatal;
use crate::structs::{KeyPackage, Response};
use crate::utils::get_json_data;
use reqwest::blocking;

// get current public ip from ipify
pub fn current_ip() -> String {
    let current_ip = blocking::get("https://api.ipify.org")
        .unwrap_or_else(|e| deblogger_fatal("Unable to get address from ipify", e.to_string()));
    current_ip.text().unwrap()
}

// everything bellow here is used to send the request and manage the response
pub fn current_dns_entry() -> String {
    let keys = get_keys();
    let target_url = build_url();
    let response = send_request(keys, target_url);
    let json_response: Response = serde_json::from_str(&response).unwrap_or_else(|e| {
        deblogger_fatal(
            format!("There was an error with the response '{}'", response),
            e.to_string(),
        )
    });

    let record = json_response.records.get(0).unwrap();
    record.content.clone()
}

fn get_keys() -> String {
    let json_data = get_json_data();
    let key_package = KeyPackage {
        secretapikey: json_data.secretapikey,
        apikey: json_data.apikey,
    };
    let payload = serde_json::to_string(&key_package)
        .unwrap_or_else(|e| deblogger_fatal("Unable to package the keys", e.to_string()));
    payload
}

fn build_url() -> String {
    let json_data = get_json_data();

    let url = format!(
        "https://api.porkbun.com/api/json/v3/dns/retrieveByNameType/{}/A/{}",
        json_data.domain, json_data.subdomain
    );
    url
}

fn send_request(keys: String, url: String) -> String {
    let api_request = blocking::Client::new();
    let response = match api_request.post(url).body(keys).send() {
        Ok(answer) => answer.text().unwrap(),
        Err(e) => deblogger_fatal("Something went wrong with the request", e.to_string()),
    };

    response
}
