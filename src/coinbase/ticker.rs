use std::time::{SystemTime, UNIX_EPOCH};
use serde::Deserialize;
use reqwest::{Client, Error};
use hmac::{Hmac, Mac};
use sha2::Sha256;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub amount: String,
    pub currency: String,
}

#[derive(Deserialize, Debug)]
pub struct Ticker {
    pub data: Data,
}

pub async fn get_ticker(ticker: &String) -> Result<Ticker, Error> {
    let api_key = std::env::var("COINBASE_API_KEY").is_ok().to_string();
    let api_secret = std::env::var("COINBASE_API_SECRET").is_ok().to_string();

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let path = format!("/v2/prices/{}/spot", ticker);
    let message = format!("{}GET{}", timestamp.to_string(), path);

    let mut hasher = HmacSha256::new_from_slice(&api_secret.into_bytes()).expect("HMAC can take key of any size");
    hasher.update(&message.into_bytes());
    let signature = format!("{:x}", hasher.finalize().into_bytes());
    
    let request_url = format!("https://api.coinbase.com{}", path);
    let response = Client::new()
        .get(request_url)
        .header("CB-ACCESS-KEY", api_key)
        .header("CB-ACCESS-TIMESTAMP", timestamp.to_string())
        .header("CB-VERSION", "2015-07-22")
        .header("CB-ACCESS-SIGN", signature)
        .send()
        .await?;

    let ticker = response.json().await?;
    Ok(ticker)
}
