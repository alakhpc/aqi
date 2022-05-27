mod structs;

use anyhow::Result;
use clap::Parser;
use dotenv_codegen::dotenv;
use structs::AqiResponse;

#[derive(Parser, Debug)]
struct Args {
    #[clap()]
    city: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let token = dotenv!("TOKEN").to_string();

    let client = reqwest::Client::builder().build()?;

    let resp = client
        .get("https://api.waqi.info/search/")
        .query(&[("keyword", &args.city), ("token", &token)])
        .send()
        .await?
        .json::<AqiResponse>()
        .await?
        .result()?;

    for station in resp.iter().filter(|station| station.aqi != "-").take(4) {
        println!("{}: {}", station.station.name, station.aqi);
    }

    Ok(())
}
