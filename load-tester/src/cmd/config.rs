use super::execute::Args;
use reqwest::Url;
use std::time::Duration;

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
