// Internal Libraries

// External Libraries
use std::{env, collections::HashMap};

use dotenv::dotenv;
use reqwest::{*, header::HeaderMap};
use serde_json::{json, Value};
use chrono::Local;

const IP_SRC: &str = "http://icanhazip.com";

#[tokio::main]
async fn main() {
    dotenv().ok();

    let flags: Vec<String> = env::args().collect();

    // Verification for all .env information
    let api_key = match env::var("CLOUDFLARE_API_KEY") {
        Ok(string) => string,
        Err(_) => {
            eprintln!("Unable to get Cloudflare API Key");
            return;
        }
    };
    let zone_id = match env::var("CLOUDFLARE_ZONE_ID") {
        Ok(string) => string,
        Err(_) => {
            eprintln!("Unable to get Cloudflare Zone ID");
            return;
        }
    };
    let email = match env::var("CLOUDFLARE_ACC_EMAIL") {
        Ok(string) => string,
        Err(_) => {
            eprintln!("Unable to get Cloudflare Account Email");
            return;
        }
    };
    let domain = match env::var("DOMAIN_NAME") {
        Ok(string) => string,
        Err(_) => {
            eprintln!("Unable to get Domain Name");
            return;
        }
    };

    // HTTP Client setup
    let client = Client::new();

    // Get External/Public IP
    let pub_ip = client.get(IP_SRC)
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();

    if flags.contains(&"-t".to_string()) {
        println!("{:?}", pub_ip.trim());

        return
    }

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
    body.insert("name", &domain);

    // Send request and get response
    let res = client
        .get(get_api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await.unwrap();

    // Get the response body and store as json
    let res_body = res.text().await.unwrap();
    let res_json: Value = serde_json::from_str(&res_body).unwrap();

    // Check success
    if let Value::Bool(false) = res_json["success"] {
        eprintln!("Unable to get from Cloudflare");
        return
    }

    let results = if let Value::Array(results) = &res_json["result"] {
        results
    } else {
        eprintln!("Unable to find any DNS entries");
        return;
    };

    let mut id = String::new();
    for result in results {
        match (&result["name"], &result["id"]) {
            (Value::String(name), Value::String(for_id)) => {
                if name == &domain {
                    id = for_id.to_string();
                    break;
                }
            }
            _ => continue,
        }
    }

    if id.is_empty() {
        eprintln!("Unable to find entry with domain name");
        return;
    }

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
    let datetime = Local::now().format("Updated on %Y-%m-%d at %H:%M:%S");

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
    println!("{} with Status: {}", datetime.to_string(), res.status());
}
