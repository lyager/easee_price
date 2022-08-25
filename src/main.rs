use std::collections::HashMap;

use std::env;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BearerResponse {
   access_token: String 
}

fn get_bearer(username: String, password: String) -> String {
    let base_url = "https://api.easee.cloud/api/accounts/login";
    let client = Client::new();
    let mut data = HashMap::new();
    data.insert("password", password);
    data.insert("userName", username);
    let response = client.post(base_url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/*+json")
        .json(&data)
        .send()
        .unwrap();
    println!("Response = {:?}", response);
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("All is well");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        _ => {
            panic!("Something unexpected happened.");
        },
    }

    let json = response.json::<BearerResponse>().unwrap();
    println!("Json response = {:?}", json.access_token);
    json.access_token
}

fn main() {
    // Get bearer
    let username = env::var("EASEE_USERNAME").expect("EASEE_USERNAME not set as environemnt variable");
    let password = env::var("EASEE_PASSWORD").expect("EASEE_PASSWORD not set as environment variable");

    let bearer = get_bearer(username, password);
    println!("Got bearer: {}", bearer);

}
