use super::config::Config;
use super::http;
use clap::Parser;
use reqwest::Client;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target URL to load test
    #[arg(short, long, default_value = "http://localhost:8080/ping")]
    pub endpoint: String,

    /// Number of virtual users
    #[arg(short, long, default_value_t = 5)]
    pub virtual_users: u32,

    /// Duration of test
    #[arg(short, long, default_value_t = 30)]
    pub duration_s: u64,

    /// HTTP method
    #[arg(short, long, default_value = "GET")]
    pub method: String,
}

pub async fn run(config: Config) -> anyhow::Result<()> {
    let client = Client::new();

    for _ in 0..config.virtual_users {
        let client = client.clone(); // uses Arc internally, cloning is cheap
        http::get_request(&client, config.endpoint.clone()).await?;
    }

    Ok(())
}
