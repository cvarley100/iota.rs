use crate::utils::{self, input_validator};
use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn were_addresses_spent_from(
    uri: &str,
    addresses: &[String],
) -> Result<WereAddressesSpentFromResponse, Error> {
    let addresses: Vec<String> = addresses
        .iter()
        .filter(|address| input_validator::is_address(address))
        .map(|address| utils::remove_checksum(address))
        .collect();
    ensure!(!addresses.is_empty(), "No valid addresses provided.");

    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "wereAddressesSpentFrom",
        "addresses": addresses,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct WereAddressesSpentFromResponse {
    duration: i64,
    error: Option<String>,
    states: Option<Vec<bool>>,
}

impl WereAddressesSpentFromResponse {
    pub fn duration(&self) -> i64 {
        self.duration
    }
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
    pub fn states(self) -> Option<Vec<bool>> {
        self.states
    }
    pub fn state(self, index: usize) -> bool {
        self.states.unwrap_or_default()[index]
    }
}