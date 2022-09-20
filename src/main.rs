use std::collections::HashMap;

use std::env;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{Result, Value}
use clap::{App, Arg};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BearerResponse {
   access_token: String 
}

fn get_current_price() {
    // http 'https://api.energidataservice.dk/datastore_search?resource_id=elspotprices&filters={"PriceArea":"DK2", "HourDK":"2022-08-25T21:00:00"}&sort=HourDK desc&fields=SpotPriceDKK' | jq .result.records\[0\].SpotPriceDKK
    let client = Client::new();
    let response = client.get("https://api.energidataservice.dk/datastore_search")
        .query(&(["resource_id", "elspotprices"],
                 ["filters", "{\"PriceArea\":\"DK2\", \"HourDK\":\"2022-08-25T21:00:00\"}"],
                 ["sort", "HourDK"],
                 ["fields", "SpotPriceDKK"]))
        .send()
        .unwrap();
    //println!("get_current_price, response {:?}", response.text());

    // Parse json
    let json = response.json::<SpotPriceRecord>().unwrap();

    println!("Price: {:?}", json);

    panic!("I'm done");
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
    let site_id= env::var("EASEE_SITE_ID").expect("EASEE_SITEID not set as environment variable");

    let matches = App::new("Easee Cost Post")
        .arg(Arg::with_name("kwh_price")
             .required(true)
             .index(1))
        .get_matches();

    let kwh_price = matches.value_of("kwh_price").unwrap();
    println!("Setting kwh price to: {}", kwh_price);

    // Get price
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
