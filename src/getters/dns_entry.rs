use crate::deblogger::{deblogger, deblogger_fatal};
use crate::structs;
use crate::utils::get_json_data;
use reqwest;
use std::time::Duration;
use tokio::time::sleep;

pub async fn dns_entry() -> String {
    let keys = get_keys();
    let target_url = build_url();

    let mut data: Option<structs::Response> = None;
    let mut i = 1;
    while i <= 4 {
        let response = send_request(&keys, &target_url).await;
        let response_unwrapping = serde_json::from_str(&response);

        match response_unwrapping {
            Ok(response_data) => {
                data = response_data;
                break;
            }
            Err(_e) => {
                if i == 4 {
                    deblogger_fatal(
                        "The server repeatedly failed to send back unexpected data.",
                        response.to_string(),
                    );
                } else {
                    deblogger(format!(
                        "The server sent back an unexpected response... Trying again [{}/3]",
                        i
                    ));
                }
            }
        };
        sleep(Duration::from_secs(3)).await;
        i += 1;
    }

    let data = data.expect(""); // this will never be None, the code will panic before that
                                // with "deblogger_fatal" but I cant seem to make the complier
                                // understand this.
    let data = data.records.get(0);
    let test = 10;
    let ip = match data {
        Some(data) => data,
        None => deblogger_fatal("The server is either offline or ignoring our requests. Please verify you have entered the correct domain and sub domain.", "The server returned nothing".to_string()
        ),
    };
    return ip.content.clone();
}

fn get_keys() -> String {
    let json_data = get_json_data();
    let key_package = structs::KeyPackage {
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

async fn send_request(keys: &String, url: &String) -> String {
    let api_request = reqwest::Client::new();
    let response = match api_request.post(url).body(keys.clone()).send().await {
        Ok(answer) => answer.text().await.unwrap(),
        Err(e) => deblogger_fatal("Something went wrong with the request", e.to_string()),
    };
    return response;
}
