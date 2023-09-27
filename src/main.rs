use std::collections::HashMap;
use std::env;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::Timelike;
use log::info;
use env_logger::Env;

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

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, fields_iter::FieldsInspect)]
#[serde(rename_all = "camelCase")]
struct DatahubPricelistRecordItem {
    #[serde(rename="ChargeOwner")]
    chargeowner: String,
    #[serde(rename="GLN_Number")]
    gln_number: String,
    #[serde(rename="ChargeType")]
    chargetype: String,
    #[serde(rename="Note")]
    note: String,
    #[serde(rename="Description")]
    description: String,
    #[serde(rename="ValidFrom")]
    validfrom: String,
    #[serde(rename="ValidTo")]
    validto: String,
    #[serde(rename="VATClass")]
    vatclass: String,
    #[serde(rename="Price1")]
    price1: f64,
    #[serde(rename="Price2")]
    price2: f64,
    #[serde(rename="Price3")]
    price3: f64,
    #[serde(rename="Price4")]
    price4: f64,
    #[serde(rename="Price5")]
    price5: f64,
    #[serde(rename="Price6")]
    price6: f64,
    #[serde(rename="Price7")]
    price7: f64,
    #[serde(rename="Price8")]
    price8: f64,
    #[serde(rename="Price9")]
    price9: f64,
    #[serde(rename="Price10")]
    price10: f64,
    #[serde(rename="Price11")]
    price11: f64,
    #[serde(rename="Price12")]
    price12: f64,
    #[serde(rename="Price13")]
    price13: f64,
    #[serde(rename="Price14")]
    price14: f64,
    #[serde(rename="Price15")]
    price15: f64,
    #[serde(rename="Price16")]
    price16: f64,
    #[serde(rename="Price17")]
    price17: f64,
    #[serde(rename="Price18")]
    price18: f64,
    #[serde(rename="Price19")]
    price19: f64,
    #[serde(rename="Price20")]
    price20: f64,
    #[serde(rename="Price21")]
    price21: f64,
    #[serde(rename="Price22")]
    price22: f64,
    #[serde(rename="Price23")]
    price23: f64,
    #[serde(rename="TransparentInvoicing")]
    transparentinvoicing: u8,
    #[serde(rename="TaxIndicator")]
    taxindicator: u8,
    #[serde(rename="ResolutionDuration")]
    resolutionduration: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DatahubPricelistRecord {
    total: u32,
    limit: u32,
    dataset: String,
    records: Vec<DatahubPricelistRecordItem>
}

// Make async
fn get_radius_charges() -> f64 {
    let client = Client::new();
    // https://api.energidataservice.dk/dataset/DatahubPricelist?offset=0&filter={%22GLN_Number%22:[%225790000705689%22],%20%22ChargeTypeCode%22:[%22DT_C_01%22]}&end=now&sort=ValidFrom%20DESC&timezone=dk
    let query = json!({
        "end": "now",
        "sort": "ValidFrom DESC",
        "timezone": "dk",
        "limit": "1",
        "filter": "{\"GLN_Number\":[\"5790000705689\"], \"ChargeTypeCode\":[\"DT_C_01\"]}",
    });
    let response = client.get("https://api.energidataservice.dk/dataset/DatahubPricelist")
        .query(&query)
        .send()
        .unwrap();

    let response_text = response.text().unwrap();

    let json_res = serde_json::from_str::<DatahubPricelistRecord>(&response_text).expect(&response_text);

    let record = &json_res.records[0];
    let hour_now = chrono::Local::now().hour();
    let price_field = format!("price{}", hour_now-1);
    println!("price_field: {}", price_field);

    let field = fields_iter::FieldsIter::new(record)
        .find(|&(name, _) | name == price_field)
        .unwrap_or_else(|| panic!("Unable to find attribute {}", price_field))
        .1
        .downcast_ref::<f64>()
        .expect("price doesn't contain type f64");


    println!("get_radius_charges, response {:?}", field);
    *field
}

fn get_current_spotprice_dkk() -> f64 {
    // http 'https://api.energidataservice.dk/datastore_search?resource_id=elspotprices&filters={"PriceArea":"DK2", "HourDK":"2022-08-25T21:00:00"}&sort=HourDK desc&fields=SpotPriceDKK' | jq .result.records\[0\].SpotPriceDKK
    let client = Client::new();
    let query = json!({
        "end": "now",
        "sort": "HourDK DESC",
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
            info!("Need to grab a new token");
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
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    // Get bearer
    let username = env::var("EASEE_USERNAME").expect("EASEE_USERNAME not set as environment variable");
    let password = env::var("EASEE_PASSWORD").expect("EASEE_PASSWORD not set as environment variable");
    let site_id= env::var("EASEE_SITE_ID").expect("EASEE_SITE_ID not set as environment variable");

    // Get price
    let radius_charges = get_radius_charges();
    let electricy_tax = 0.6970;
    let energinet_charges = 0.058 + 0.054;  // nettarif/transmissionstarif + systemtarif
    let total_charge = radius_charges + energinet_charges + electricy_tax;

    let kwh_price = get_current_spotprice_dkk();
    let vat = 1.20;
    let total_wo_vat = kwh_price + total_charge;
    info!("Detailed:");
    info!(" - radius_charges: {}", radius_charges);
    info!(" - electricy_tax: {}", electricy_tax);
    info!(" - energinet_charges: {}", energinet_charges);
    info!(" - spotprice now {}", kwh_price);
    info!(" - vat {}", vat);
    info!("");
    info!("Current price in DKK w/o VAT per kwh: {}, charges: {}", kwh_price, total_charge);
    info!("Total: {} w/ VAT: {}", total_wo_vat, total_wo_vat * vat);

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
