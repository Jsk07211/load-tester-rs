use crate::config::Config;
use crate::http::get_request;
use crate::metrics::{RunStatistics, print_summary};
use clap::Parser;
use reqwest::{Client, Url};
use std::{
    sync::{Arc, atomic::Ordering},
    time::Instant,
};

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

#[derive(Clone)]
struct SharedState {
    client: Client,
    url: Url,
    stats: Arc<RunStatistics>,
}

pub async fn run(config: Config) -> anyhow::Result<()> {
    let shared = SharedState {
        client: Client::new(), // uses Arc internally, cloning is cheap
        url: config.endpoint,
        stats: Arc::new(RunStatistics::default()),
    };

    let mut tasks = Vec::new();
    let deadline = Instant::now() + config.duration;

    for _ in 0..config.virtual_users {
        let shared = shared.clone();

        // tokio spawn does not block outer loop
        tasks.push(tokio::spawn(async move {
            while Instant::now() < deadline {
                let start = Instant::now();
                let result = get_request(&shared.client, &shared.url).await;
                let elapsed = start.elapsed();

                // Populate stats
                if result.is_ok() {
                    // Ordering::Relaxed: No ordering guarantee at all beyond the atomic operation itself being atomic
                    shared.stats.success_count.fetch_add(1, Ordering::Relaxed);
                } else {
                    shared.stats.error_count.fetch_add(1, Ordering::Relaxed);
                }

                shared.stats.latencies.lock().unwrap().push(elapsed);
            }
        }));
    }

    for task in tasks {
        task.await?;
    }

    print_summary(&shared.stats, config.duration);

    Ok(())
}
