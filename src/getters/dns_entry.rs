use crate::deblogger::{deblogger, deblogger_fatal};
use crate::structs;
use crate::utils::get_json_data;
use reqwest;

pub async fn dns_entry() -> String {
    let keys = get_keys();
    let target_url = build_url();

    let response = send_request(&keys, &target_url).await;
    let response_unwrapping = serde_json::from_str(&response);
    let data: structs::Response;

    match response_unwrapping {
        Ok(response_data) => {
            data = response_data;
        }
        Err(_e) => {
            handle_unexecpted_responce(response);
            unreachable!()
        }
    };

    if data.records.is_empty() {
        deblogger("The records came back empty... A new subdomain will be created.");
        return String::from("0.0.0.0");
    }

    let data = data.records.get(0).unwrap();
    return data.content.clone();
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

fn handle_unexecpted_responce(response: String) {
    let status: structs::Status =
        serde_json::from_str(response.as_str()).unwrap_or_else(|_error| {
            deblogger_fatal(
                "The server sent back a response in an unknown format",
                response.clone(),
            );
        });

    if status.status == "ERROR" {
        if let Some(message) = status.message {
            if message.contains("Invalid API key") {
                deblogger_fatal(
                    "The server returned an error",
                    "Invalid API key".to_string(),
                );
            } else if message.contains("Invalid domain") {
                deblogger_fatal("The server returned an error", "Invalid domain".to_string());
            } else {
                deblogger_fatal("The server returned an unforeseen error", response);
            }
        }
    }
}
