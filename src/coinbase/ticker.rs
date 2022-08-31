use reqwest::{Client, Error};

pub enum Granularity {
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    OneHour,
    SixHours,
    OneDay,
}

// Vec<(timestamp, price_low, price_high, price_open, price_close)>
pub async fn get_ticker(
    ticker: &String,
    granularity: Granularity,
) -> Result<Vec<(u32, f32, f32, f32, f32, f32)>, Error> {
    let translated = match granularity {
        Granularity::OneMinute => 60,
        Granularity::FiveMinutes => 300,
        Granularity::FifteenMinutes => 900,
        Granularity::OneHour => 3600,
        Granularity::SixHours => 21600,
        Granularity::OneDay => 86400,
    };

    let path = format!("/products/{}/candles", ticker);
    let request_url = format!(
        "https://api.exchange.coinbase.com{}?granularity={}",
        path, translated
    );
    let response = Client::new()
        .get(request_url)
        .header("User-Agent", "node-fetch")
        .send()
        .await?;

    let candles = response.json().await?;
    Ok(candles)
}
