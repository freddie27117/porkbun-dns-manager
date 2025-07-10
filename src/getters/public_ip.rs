use crate::utilities::deblogger::{deblogger, deblogger_fatal};
use reqwest;
use std::time;
use tokio::time::sleep;

// get current public ip from ipify
pub async fn current_ip() -> String {
    let mut error: String = String::new();
    let mut i = 1;
    while i <= 4 {
        let request = fetch_ip().await;

        match request {
            Ok(response) => {
                let ip = response.text().await.unwrap();
                return ip;
            }
            Err(e) => {
                if i == 4 {
                    error = e.to_string();
                    break;
                } else {
                    deblogger(format!(
                        "Unable to get IP address from ipify... Trying again in 10 seconds. [{}/3]",
                        i
                    ));
                    sleep(time::Duration::from_secs(10)).await
                }
            }
        }

        i += 1;
    }

    deblogger_fatal(
        "Attempts to get IP address from ipify repeatedly failed.",
        error,
    );
}

async fn fetch_ip() -> reqwest::Result<reqwest::Response> {
    return reqwest::get("https://api.ipify.org").await;
}
