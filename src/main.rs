use std::collections::HashMap;

use std::env;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use clap::{App, Arg};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BearerResponse {
   access_token: String 
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SpotPriceRecordItem {
    #[serde(rename="HourUTC")]
    hourutc: String,
    #[serde(rename="HourDK")]
    hourdk: String,
    #[serde(rename="PriceArea")]
    pricearea: String,
    #[serde(rename="SpotPriceDKK")]
    spotpricedkk: Option<f64>,
    #[serde(rename="SpotPriceEUR")]
    spotpriceeur: Option<f64>
}
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SpotPriceRecord {
    total: u32,
    limit: u32,
    dataset: String,
    records: Vec<SpotPriceRecordItem>
}

fn get_current_price() {
    // http 'https://api.energidataservice.dk/datastore_search?resource_id=elspotprices&filters={"PriceArea":"DK2", "HourDK":"2022-08-25T21:00:00"}&sort=HourDK desc&fields=SpotPriceDKK' | jq .result.records\[0\].SpotPriceDKK
    let client = Client::new();
    let response = client.get("https://api.energidataservice.dk/dataset/Elspotprices")
        .query(&(["end", "2023-04-16"],
                 ["filters", "{\"PriceArea\":[\"DK2\"]}"],
                 ["sort", "HourDK"],
                 ["limit", "23"]))
        .send()
        .unwrap();

    assert!(response.status().is_success());

    let response_text = response.text().unwrap();
    println!("get_current_price, response {:?}", response_text);

    // Parse json
    let json_res = serde_json::from_str::<SpotPriceRecord>(&response_text).expect(&response_text);
    //let json_res = response.json::<SpotPriceRecord>().unwrap();
    println!("Dataset: {}", json_res.total);
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
    let site_id= env::var("EASEE_SITE_ID").expect("EASEE_SITE_ID not set as environment variable");

    // Get price
    let kwh_price = "3";
    get_current_price();

    // Login to Easee
    let bearer = get_bearer(username, password);
    println!("Got bearer: {}", bearer);
    // Set price
    let mut data = HashMap::new();
    data.insert("currencyId", "dkk");
    data.insert("costPerKWh", kwh_price);
    let client = Client::new();
    let response = client.post(format!("https://api.easee.cloud/api/sites/{}/price", site_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/*+json")
        .header("Authorization", format!("Bearer {}", bearer))
        .json(&data)
        .send()
        .unwrap();
    println!("Response = {:?}", response);

}
