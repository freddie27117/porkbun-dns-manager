use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JSONdata {
    pub domain: String,
    pub subdomain: String,
    pub secretapikey: String,
    pub apikey: String,
}
#[derive(Serialize)]
pub struct KeyPackage {
    pub secretapikey: String,
    pub apikey: String,
}
#[derive(Deserialize)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub content: String,
    pub ttl: String,
    pub prio: String,
    pub notes: String,
}
#[derive(Deserialize)]
pub struct Response {
    pub status: String,
    pub cloudflare: String,
    pub records: Vec<Record>,
}
