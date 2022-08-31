mod analytics;
mod coinbase;

use clap::Parser;

use crate::analytics::{exponential_moving_average, get_percentage_difference, moving_average};
use crate::coinbase::get_ticker;

#[derive(Parser)]
struct Cli {
    ticker: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let ticker = &args.ticker;
    println!();
    println!();
    println!("Ticker: {}", ticker);
    println!();

    let result = get_ticker(ticker, coinbase::Granularity::OneMinute)
        .await
        .ok()
        .unwrap();
    let result_per_day = get_ticker(ticker, coinbase::Granularity::OneDay)
        .await
        .ok()
        .unwrap();

    let current_price = result.first().unwrap();
    let closing_prices: Vec<f32> = result.iter().map(|&x| x.4).collect();
    let current_close_price: f32 = current_price.4;

    println!("Price: {:.2}", current_close_price);
    println!();
    println!("MA20: {:.2}", moving_average(&closing_prices, 20));
    println!("MA20: {:.2}", moving_average(&closing_prices, 50));
    println!(
        "EMA20: {:.2}",
        exponential_moving_average(&closing_prices, 20)
    );
    println!(
        "EMA50: {:.2}",
        exponential_moving_average(&closing_prices, 50)
    );

    let closing_daily_prices: Vec<f32> = result_per_day.iter().map(|&x| x.4).collect();

    println!();
    println!(
        "Price % diff 1 day {:.2}%",
        get_percentage_difference(&current_close_price, closing_daily_prices.get(1).unwrap())
    );
    println!(
        "Price % diff 7 days {:.2}%",
        get_percentage_difference(&current_close_price, closing_daily_prices.get(7).unwrap())
    );
    println!(
        "Price % diff 30 days {:.2}%",
        get_percentage_difference(&current_close_price, closing_daily_prices.get(30).unwrap())
    );
}
