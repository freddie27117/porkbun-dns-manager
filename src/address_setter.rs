use crate::deblogger::deblogger_fatal;
use crate::structs::{Status, UpdateRequest};
use crate::utils::get_json_data;
use reqwest::blocking;

pub fn dns_record(ip: String) {
    let payload = build_request(ip);
    let url = build_url();
    send_request(payload, url);
}

fn build_request(ip: String) -> String {
    let json_data = get_json_data();

    let request = UpdateRequest {
        secretapikey: json_data.secretapikey,
        apikey: json_data.apikey,
        content: ip,
        ttl: json_data.ttl,
    };

    let request_json = serde_json::to_string(&request).unwrap_or_else(|e| {
        deblogger_fatal("Could not convert the request to json", e.to_string())
    });
    request_json
}

fn build_url() -> String {
    let json_data = get_json_data();

    let url = format!(
        "https://porkbun.com/api/json/v3/dns/editByNameType/{}/A/{}",
        json_data.domain, json_data.subdomain
    );
    url
}

fn send_request(payload: String, target_url: String) {
    let request = blocking::Client::new();

    let response = request
        .post(target_url)
        .body(payload)
        .send()
        .unwrap_or_else(|e| {
            deblogger_fatal("There was an error sending the request", e.to_string())
        });

    let status = response.text().unwrap();

    let status: Status = serde_json::from_str(status.as_str()).unwrap_or_else(|e| {
        deblogger_fatal(
            format!(
                "Could not decipher the response from the server. The response was '{}'",
                status
            ),
            e.to_string(),
        )
    });

    if status.status != "SUCCESS" {
        deblogger_fatal(
            "The returned code was something other than success",
            status.status,
        )
    }
}
