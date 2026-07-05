use clap::Parser;
/// Validates input
use reqwest::Url;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target URL to load test
    #[arg(short, long, default_value = "http://localhost:8080/ping")]
    pub endpoint: String,

    /// Number of virtual users (concurrent in-flight requests, sustained for the full test duration)
    #[arg(short, long, default_value_t = 5)]
    pub virtual_users: u32,

    /// Duration of test
    #[arg(short, long, default_value_t = 5)]
    pub duration_s: u64,

    /// HTTP method
    #[arg(short, long, default_value = "GET")]
    pub method: String,
}

pub struct Config {
    pub endpoint: Url,
    pub virtual_users: u32,
    pub duration_s: Duration,
    pub method: http::Method,
}

impl TryFrom<Args> for Config {
    type Error = anyhow::Error; // Defines Self::Error

    fn try_from(args: Args) -> anyhow::Result<Self> {
        Ok(Config {
            endpoint: Url::parse(&args.endpoint)?,
            virtual_users: args.virtual_users,
            duration_s: Duration::from_secs(args.duration_s),
            method: args.method.parse()?, // type inference
        })
    }
}
