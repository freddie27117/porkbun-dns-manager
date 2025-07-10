use crate::utilities::deblogger::deblogger_fatal;
use crate::utilities::structs::{Status, UpdateRequest};
use crate::utilities::utils::get_json_data;
use reqwest;

pub async fn update_dns_record(ip: String) {
    let payload = build_request(ip);
    let url = build_url();
    send_request(payload, url).await;
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
        deblogger_fatal(
            "An error occurred when trying to package the DNS update request.",
            e,
        )
    });
    request_json
}

fn build_url() -> String {
    let json_data = get_json_data();

    let url = format!(
        "https://api.porkbun.com/api/json/v3/dns/editByNameType/{}/A/{}",
        json_data.domain, json_data.subdomain
    );
    url
}

async fn send_request(payload: String, target_url: String) {
    let request = reqwest::Client::new();

    let response = request
        .post(target_url)
        .body(payload)
        .send()
        .await
        .unwrap_or_else(|e| {
            deblogger_fatal("An error occurred when sending the DNS update request.", e)
        });

    let response = response.text().await.unwrap();

    let result: Status = serde_json::from_str(response.as_str()).unwrap_or_else(|e| {
        deblogger_fatal(
            format!(
                "Could not decipher the response from the server. The response was '{}'",
                response
            ),
            e,
        )
    });

    if result.status != "SUCCESS" {
        deblogger_fatal("Updating the DNS entry failed.", format!("{:#?}", result))
    }
}
