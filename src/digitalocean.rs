use std::env;

use serde::{Deserialize};
use serde_json::json;

use ureq;

static API_BASE_URL: &str = "https://api.digitalocean.com/v2";

pub struct Config {
    pub api_token: String,
    pub hostname: String,
}

impl Config {
    pub fn new_from_environment() -> Config {
        let api_token =
            match env::var("DO_API_TOKEN") {
                Ok(api_token) => api_token,
                _ => panic!("No DO_API_TOKEN set in environment"),
            };

        let hostname =
            match env::var("DYN_HOSTNAME") {
                Ok(hostname) => hostname,
                _ => panic!("No DYN_HOSTNAME set in environment"),
            };

        Config {
            api_token,
            hostname,
        }
    }

    pub fn header_value(&self) -> String {
        format!("Bearer {}", self.api_token)
    }
}


#[derive(Debug, Clone, Deserialize)]
struct DomainRecord {
    #[serde(rename = "type")]
    pub record_type: String,
    pub id: u32,
    pub name: String,
}

pub fn build_request(config: &Config, method: &str, uri: &str) -> ureq::Request {
    let url = format!("{}{}", API_BASE_URL, uri);
    let auth_header = config.header_value();

    ureq::request(method, &url)
        .set("authorization", &auth_header)
        .set("content-type", "application/json")
        .build()
}

pub fn list_records(config: &Config) -> ureq::Response {
    let uri = format!("/domains/{}/records", config.hostname);
    build_request(config, "GET", &uri)
        .query("per_page", "200")
        .call()
}

pub fn find_record_id(config: &Config) -> Option<u32> {
    let resp = list_records(config);

    if ! resp.ok() {
        panic!("Failed to list records ID");
    }

    let records_json = resp.into_json().unwrap();
    let array_json = records_json["domain_records"].to_string();
    let records: Vec<DomainRecord> = serde_json::from_str(&array_json).unwrap();

    for record in records.iter() {
        if record.record_type != "A" {
            continue;
        }

        if record.name == "@" {
            return Some(record.id);
        }
    }

    None
}

pub fn update_record(config: &Config, record_id: u32, ip: &str) -> ureq::Response {
    let url = format!("/domains/{}/records/{}", config.hostname, record_id);
    let payload = json!({
        "data": ip
    });

    build_request(config, "PUT", &url)
        .send_string(&payload.to_string())
}

