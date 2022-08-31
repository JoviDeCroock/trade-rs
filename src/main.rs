mod coinbase;
mod analytics;

use clap::Parser;

use crate::coinbase::get_ticker;
use crate::analytics::{moving_average, exponential_moving_average};

#[derive(Parser)]
struct Cli {
    ticker: String,
}

fn get_percentage_difference(first: &f32, second: &f32) -> f32 {
    if first == second {
        return 0.0;
    }

    ((first / second) * 100.0) - 100.0
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let ticker = &args.ticker;
    println!("Ticker: {}", ticker);
    println!();

    let result = get_ticker(ticker, coinbase::Granularity::OneMinute).await.ok().unwrap();
    let result_per_day = get_ticker(ticker, coinbase::Granularity::OneDay).await.ok().unwrap();

    let current_price = result.first().unwrap();
    let closing_prices: Vec<f32> = result.iter().map(|&x| x.4).collect();
    let current_close_price: f32 = current_price.4;

    println!("Price: {}", current_close_price);
    println!();
    println!("MA20: {}", moving_average(&closing_prices, 20));
    println!("MA20: {}", moving_average(&closing_prices, 50));
    println!("EMA20: {}", exponential_moving_average(&closing_prices, 20));
    println!("EMA50: {}", exponential_moving_average(&closing_prices, 50));

    let closing_daily_prices: Vec<f32> = result_per_day.iter().map(|&x| x.4).collect();
    let price_1_day_ago: &f32 = closing_daily_prices.get(1).unwrap();
    let price_7_days_ago: &f32 = closing_daily_prices.get(7).unwrap();
    let price_30_days_ago: &f32 = closing_daily_prices.get(30).unwrap();

    println!();
    println!("Price % diff 1 day {:.2}%", get_percentage_difference(&current_close_price, price_1_day_ago));
    println!("Price % diff 7 days {:.2}%", get_percentage_difference(&current_close_price, price_7_days_ago));
    println!("Price % diff 30 days {:.2}%", get_percentage_difference(&current_close_price, price_30_days_ago));
}
