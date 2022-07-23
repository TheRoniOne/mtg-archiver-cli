use reqwest::blocking::{Client, Response};
use reqwest::Result;

use serde_json::Value;

fn retrieve_bulk_data_info(client: &Client) -> Result<Response> {
    client.get("https://api.scryfall.com/bulk-data").send()
}

fn retrieve_cards(bulk_data_info: Value, client: &Client) {
    let uri = bulk_data_info["data"][0]["download_uri"].as_str().unwrap();
    let response = client.get(uri).send().unwrap();
    println!("Success! {:?}", response.text())
}

pub fn retrieve_data() {
    let client = Client::new();
    let bulk_data_info: Value = retrieve_bulk_data_info(&client).unwrap().json().unwrap();
    println!("data = {}", bulk_data_info["data"][0]["download_uri"]);

    retrieve_cards(bulk_data_info, &client)
}
