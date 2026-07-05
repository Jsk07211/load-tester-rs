use super::config::Config;
use crate::cmd::http::get_request;
use clap::Parser;
use reqwest::{Client, Url};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
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
    request_count: Arc<AtomicU64>,
}

pub async fn run(config: Config) -> anyhow::Result<()> {
    let shared = SharedState {
        client: Client::new(), // uses Arc internally, cloning is cheap
        url: config.endpoint,
        request_count: Arc::new(AtomicU64::new(0)),
    };

    let mut tasks = Vec::new();
    let deadline = Instant::now() + config.duration_s;

    for _ in 0..config.virtual_users {
        let shared = shared.clone();

        // tokio spawn does not block outer loop
        tasks.push(tokio::spawn(async move {
            while Instant::now() < deadline {
                if get_request(&shared.client, &shared.url).await.is_ok() {
                    shared.request_count.fetch_add(1, Ordering::Relaxed); // no ordering guarantee at all beyond the atomic operation itself being atomic
                }
            }
        }));
    }

    for task in tasks {
        task.await?;
    }

    println!(
        "Total requests: {}",
        shared.request_count.load(Ordering::Relaxed)
    );

    Ok(())
}
