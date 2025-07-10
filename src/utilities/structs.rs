use serde::{Deserialize, Serialize};

// for receiving
#[derive(Deserialize, Serialize, Debug)]
pub struct JSONdata {
    pub domain: String,
    pub subdomain: String,
    pub ttl: String,
    pub secretapikey: String,
    pub apikey: String,
}
#[derive(Serialize)]
pub struct KeyPackage {
    pub secretapikey: String,
    pub apikey: String,
}
#[derive(Deserialize, Debug)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub content: String,
    pub ttl: String,
    pub prio: String,
    pub notes: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Response {
    pub status: String,
    pub cloudflare: String,
    pub records: Vec<Record>,
}

// for sending
#[derive(Serialize)]
pub struct UpdateRequest {
    pub secretapikey: String,
    pub apikey: String,
    pub content: String,
    pub ttl: String,
}

#[derive(Serialize)]
pub struct CreateRequest {
    pub secretapikey: String,
    pub apikey: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub content: String,
    pub ttl: String,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub status: String,
    pub message: Option<String>,
}
