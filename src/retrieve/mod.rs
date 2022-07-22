use reqwest::blocking::{Client, Response};
use reqwest::Result;

pub fn retrieve_bulk_data_info() -> Result<Response> {
    let client = Client::new();
    client.get("https://api.scryfall.com/bulk-data").send()
}
