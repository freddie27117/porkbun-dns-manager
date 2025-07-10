use crate::deblogger::deblogger_fatal;
use crate::structs::{CreateRequest, Status, UpdateRequest};
use crate::utils::get_json_data;
use reqwest;

pub async fn create_dns_record(ip: String) {
    let payload = build_request(ip);
    let url = build_url();
    send_request(payload, url).await;
}

fn build_request(ip: String) -> String {
    let json_data = get_json_data();
    let request_json: String;

    let request = CreateRequest {
        secretapikey: json_data.secretapikey,
        apikey: json_data.apikey,
        name: json_data.subdomain,
        type_: "A".to_string(),
        content: ip,
        ttl: json_data.ttl,
    };

    request_json = serde_json::to_string(&request).unwrap_or_else(|e| {
        deblogger_fatal("Could not convert the request to json", e.to_string())
    });

    request_json
}

fn build_url() -> String {
    let json_data = get_json_data();
    let url: String;
    url = format!(
        "https://api.porkbun.com/api/json/v3/dns/create/{}",
        json_data.domain
    );
    return url;
}

async fn send_request(payload: String, target_url: String) {
    let request = reqwest::Client::new();

    let response = request
        .post(target_url)
        .body(payload)
        .send()
        .await
        .unwrap_or_else(|e| {
            deblogger_fatal("There was an error sending the request", e.to_string())
        });

    let response = response.text().await.unwrap();

    let result: Status = serde_json::from_str(response.as_str()).unwrap_or_else(|e| {
        deblogger_fatal(
            format!(
                "Could not decipher the response from the server. The response was '{}'",
                response
            ),
            e.to_string(),
        )
    });

    if result.status != "SUCCESS" {
        deblogger_fatal("Creating the DNS entry failed.", result.status)
    }
}
