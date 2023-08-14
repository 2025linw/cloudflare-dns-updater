// Internal Libraries

// External Libraries
use std::{env, collections::HashMap};

use dotenv::dotenv;
use reqwest::{*, header::HeaderMap};
use serde_json::json;

const IP_SRC: &str = "http://whatismyip.akamai.com";

async fn get_body(url: String) -> reqwest::Result<String> {
    let body = get(url)
    .await?
    .text()
    .await?;

    Ok(body)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Verification for all .env information
    let api_key = env::var("CLOUDFLARE_API_KEY").expect("Need Cloudflare API Key");
    let zone_id = env::var("CLOUDFLARE_ZONE_ID").expect("Need Cloudflare Zone ID");
    let email = env::var("CLOUDFLARE_ACC_EMAIL").expect("Need Cloudflare Account Email");

    // Get External/Public IP
    let pub_ip = get_body(IP_SRC.to_string()).await.unwrap();

    println!("External IP: {}", pub_ip);

    // HTTP Client setup
    let client = Client::new();

    // Get ID of DNS Record
    let get_api_url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", zone_id);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Auth-Email", email.parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());

    let mut body = HashMap::new();
    body.insert("name", "***REMOVED***");

    let res = client
        .get(get_api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await.unwrap();

    let res_body = res.text().await.unwrap();

    let start = res_body.find("id\":\"").unwrap() + 5;
    let end = res_body.find("\",\"zone_id\"").unwrap() - start;

    let id = match (match res_body.split_at(start) {
        (_, bot) => {
            bot.to_string()
        }
    }).split_at(end) {
        (top, _) => {
            top.to_string()
        }
    };

    // Put new IP Address for DNS entry
    let put_api_url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_id, id);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Auth-Email", email.parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());

    let body = json!(
        {
            "content": pub_ip,
            "name": "***REMOVED***",
            "type": "A",
            "proxied": false,
        }
    );
    
    let res = client
        .put(put_api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await.unwrap();

    println!("Status: {}", res.status());
}
