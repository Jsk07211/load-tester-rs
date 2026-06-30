use anyhow::Result;
use clap::Parser;
use load_tester::cmd::execute::Args;
use reqwest::Url;
use reqwest::header::{AUTHORIZATION, USER_AGENT};

async fn get_request(url: Url) -> Result<(), anyhow::Error> {
    let response = reqwest::get(url).await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let url = Url::parse(&args.endpoint)?;

    println!("{:?}", args);

    match args.method.as_str() {
        "GET" => get_request(url).await?,
        _ => todo!()
    }

    Ok(())
}
