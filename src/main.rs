// Internal Libraries

// External Libraries
use std::{env, collections::HashMap};

use dotenv::dotenv;
use reqwest::{*, header::HeaderMap};
use serde_json::json;
use chrono::Local;

const IP_SRC: &str = "http://whatismyip.akamai.com";

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Verification for all .env information
    let api_key = env::var("CLOUDFLARE_API_KEY").expect("Need Cloudflare API Key");
    let zone_id = env::var("CLOUDFLARE_ZONE_ID").expect("Need Cloudflare Zone ID");
    let email = env::var("CLOUDFLARE_ACC_EMAIL").expect("Need Cloudflare Account Email");
    let domain = env::var("DOMAIN_NAME").expect("Need target domain name");

    // HTTP Client setup
    let client = Client::new();

    // Get External/Public IP
    let pub_ip = client.get(IP_SRC)
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();

    //
    // GET Request
    //

    // API URL
    let get_api_url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", zone_id);

    // GET headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Auth-Email", email.parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());

    // GET body
    let mut body = HashMap::new();
    body.insert("name", "saphynet.io");

    // Send request and get response
    let res = client
        .get(get_api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await.unwrap();

    // Get the response body
    let res_body = res.text().await.unwrap();

    // Get the index of the start and end of the id
    let start = res_body.find("id\":\"").unwrap() + 5;
    let end = res_body.find("\",\"zone_id\"").unwrap() - start;

    // Extract the DNS entry ID
    let id = match (match res_body.split_at(start) {
        (_, bot) => {
            bot.to_string()
        }
    }).split_at(end) {
        (top, _) => {
            top.to_string()
        }
    };

    //
    // PUT request
    //

    // API URL
    let put_api_url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_id, id);

    // PUT headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Auth-Email", email.parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());

    // Comment for last update date and time
    let datetime = Local::now().format("Updated on %F at %R");

    // PUT body
    let body = json!(
        {
            "content": pub_ip,
            "name": domain,
            "type": "A",
            "proxied": false,
            "comment": datetime.to_string(),
        }
    );

    // Send request and get response
    let res = client
        .put(put_api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await.unwrap();

    // Output status
    println!("Status: {}", res.status());
}
