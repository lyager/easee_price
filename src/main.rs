use std::collections::HashMap;
use std::env;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{Datelike, Timelike};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BearerResponse {
   access_token: String 
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SetCharchingPrice {
    #[serde(rename="currencyId")]
    currency_id: String,
    #[serde(rename="costPerKwh")]
    cost_per_kwh: Option<f64>,
    #[serde(rename="costPerKwhExcludeVat")]
    cost_per_kwh_exclude_vat: Option<f64>,
    vat: f64
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

fn calculate_tax_dkk() -> f64 {
    // Ref: https://radiuselnet.dk/elnetkunder/tariffer-og-netabonnement/
    struct AnualPeriod {
        low: f64,
        high: f64,
        peak: f64
    }
    let winter = AnualPeriod { low: 15.09, high: 45.28, peak: 135.84 };
    let summer = AnualPeriod { low: 15.09, high: 22.64, peak: 58.87 };

    let current_time = chrono::Utc::now();
    let month = current_time.month();
    let hour = current_time.hour();
    let res_in_ore = match hour {
        0..=5 => match month {
            4..=9 => summer.low,
            _ => winter.low,
        }
        6..=16 => match month {
            4..=9 => summer.high,
            _ => winter.high,
        }
        17..=20 => match month {
            4..=9 => summer.peak,
            _ => winter.peak,
        }
        21..=24 => match month {
            4..=9 => summer.high,
            _ => winter.high,
        }
        _ => panic!("Hour out of range"),
    };
    res_in_ore / 100.
}
    
fn get_current_spotprice_dkk() -> f64 {
    // http 'https://api.energidataservice.dk/datastore_search?resource_id=elspotprices&filters={"PriceArea":"DK2", "HourDK":"2022-08-25T21:00:00"}&sort=HourDK desc&fields=SpotPriceDKK' | jq .result.records\[0\].SpotPriceDKK
    let client = Client::new();
    let query = json!({
        "start": "now",
        "sort": "HourDK",
        "limit": "1",
        "filter": "{\"PriceArea\":[\"DK2\"]}",
    });
    let response = client.get("https://api.energidataservice.dk/dataset/Elspotprices")
        .query(&query)
        .send()
        .unwrap();

    assert!(response.status().is_success());

    let response_text = response.text().unwrap();
    //println!("get_current_price, response {:?}", response_text);

    // Parse json
    let json_res = serde_json::from_str::<SpotPriceRecord>(&response_text).expect(&response_text);
    json_res.records[0].spotpricedkk.unwrap() / 1000.
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
    //println!("Response = {:?}", response);
    match response.status() {
        reqwest::StatusCode::OK => { }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        _ => {
            panic!("Something unexpected happened.");
        },
    }

    let json = response.json::<BearerResponse>().unwrap();
    //println!("Json response = {:?}", json.access_token);
    json.access_token
}

fn main() {
    // Get bearer
    let username = env::var("EASEE_USERNAME").expect("EASEE_USERNAME not set as environemnt variable");
    let password = env::var("EASEE_PASSWORD").expect("EASEE_PASSWORD not set as environment variable");
    let site_id= env::var("EASEE_SITE_ID").expect("EASEE_SITE_ID not set as environment variable");

    // Get price
    let kwh_price: f64 = get_current_spotprice_dkk();
    let tax = calculate_tax_dkk();
    let vat = 1.25;
    let total_wo_vat = kwh_price + tax;
    println!("Current price in DKK w/o VAT per kwh: {} add tax: {}", kwh_price, tax);
    println!("Total w/ VAT: {}", total_wo_vat * vat);

    // Login to Easee
    let bearer = get_bearer(username, password);
    //println!("Got bearer: {}", bearer);
    // Set price
    let price = SetCharchingPrice {
        currency_id: "DKK".to_string(),
        cost_per_kwh: Some(total_wo_vat * vat),
        cost_per_kwh_exclude_vat: Some(total_wo_vat),
        vat: vat};
    let client = Client::new();
    let response = client.post(format!("https://api.easee.cloud/api/sites/{}/price", site_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/*+json")
        .header("Authorization", format!("Bearer {}", bearer))
        .json(&price)
        .send()
        .unwrap();
    response.error_for_status().unwrap();
}
