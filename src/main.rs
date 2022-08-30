mod coinbase;

use clap::Parser;

use crate::coinbase::get_ticker;

#[derive(Parser)]
struct Cli {
    ticker: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let ticker = &args.ticker;
    println!("Ticker: {}", ticker);

    let result = get_ticker(ticker, coinbase::Granularity::OneMinute).await.ok().unwrap();
    dbg!(result.first());
}
