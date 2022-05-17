use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct CompanyQuote {
    c: f64, //Current Price
    h: f64, //High Price of Day
    l: f64, //Low Price of Day
    o: f64, //Open Price of Day
    pc: f64, //Price change of day
    t: i128, //timestamp for returned
}

impl CompanyQuote {
    async fn get(symbol: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;

        Ok(res)
    }
}


impl 

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let api_key = "ca1f2uaad3i6tbvcjrc0".to_string();
    let args: Vec<String> = env::args().collect();
    let mut symbol: String = "AAPL".to_string();

    if args.len() < 2 {
        println!("Since you didn't specify a company symbol, it has defaulted to AAPL.");
    } else {
        symbol = args[1].clone();
    }

    let res = CompanyQuote::get(&symbol, &api_key).await?;
    println!("{}'s Stock Data: {:?}", symbol, res);
    Ok(())
}

