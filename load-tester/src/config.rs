use crate::payload::PayloadSpec;
use clap::Parser;
/// Validates input
use reqwest::Url;
use std::{env::current_dir, fs, time::Duration};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target URL to load test
    #[arg(short, long, default_value = "http://localhost:8080/person")]
    pub endpoint: String,

    /// Number of virtual users (concurrent in-flight requests, sustained for the full test duration)
    #[arg(short, long, default_value_t = 1)]
    pub virtual_users: u32,

    /// Duration of test
    #[arg(short, long, default_value_t = 3.0)]
    pub duration_s: f64,

    /// Per-task timeout duration
    #[arg(short, long, default_value_t = 3.0)]
    pub timeout_s: f64,

    /// HTTP method
    #[arg(short, long, default_value = "POST")]
    pub method: String,

    /// Filepath to payload content
    #[arg(short, long, default_value = None)]
    pub file_path: Option<String>,
}

pub struct Config {
    pub endpoint: Url,
    pub virtual_users: u32,
    pub duration: Duration,
    pub timeout: Duration,
    pub method: reqwest::Method,
    pub payload: Option<PayloadSpec>,
}

impl TryFrom<Args> for Config {
    type Error = anyhow::Error; // Defines Self::Error

    fn try_from(args: Args) -> anyhow::Result<Self> {
        let mut config = Config {
            endpoint: Url::parse(&args.endpoint)?,
            virtual_users: args.virtual_users,
            duration: Duration::from_secs_f64(args.duration_s),
            timeout: Duration::from_secs_f64(args.timeout_s),
            method: args.method.parse()?, // type inference
            payload: None,
        };

        if let Some(file_path) = args.file_path {
            let mut path = current_dir()?;
            path.push(file_path);
            let payload = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Should have been able to read the file at {:?}", path));

            config.payload = Some(PayloadSpec::Json(serde_json::from_str(&payload)?));
        }

        Ok(config)
    }
}
