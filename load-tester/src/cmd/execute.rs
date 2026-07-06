use crate::config::Config;
use crate::metrics::get_summary;
use crate::worker::{self, SharedState};
use clap::Parser;

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

pub async fn run(config: Config) -> anyhow::Result<()> {
    let initial_state = SharedState::try_from(&config)?;
    let shared = worker::worker_loop(&config, initial_state).await?;
    let summary = get_summary(&shared.metrics, config.duration);
    print!("{}", summary.report());

    Ok(())
}
