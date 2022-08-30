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

    // TODO: check if valid ticker with match
    let result = get_ticker(ticker).await;
    match result {
        Ok(ticker) => println!("{}", ticker.data.amount),
        Err(e) => println!("{}", e.to_string()),
    }
}
