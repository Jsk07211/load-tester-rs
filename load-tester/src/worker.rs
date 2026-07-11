use crate::config::Config;
use crate::http::get_request;
use crate::metrics::RunMetrics;
use reqwest::{Client, Url};
use std::time::Instant;
use tokio::time::timeout;

#[derive(Clone)]
pub struct SharedState {
    pub client: Client,
    pub url: Url,
}

impl TryFrom<&Config> for SharedState {
    type Error = anyhow::Error; // Defines Self::Error

    fn try_from(config: &Config) -> anyhow::Result<Self> {
        Ok(SharedState {
            client: Client::new(),
            url: config.endpoint.clone(),
        })
    }
}

pub async fn worker_loop(
    config: &Config,
    shared: SharedState,
) -> anyhow::Result<RunMetrics, anyhow::Error> {
    let run_start = Instant::now();
    let mut tasks = Vec::new();
    let request_timeout = config.timeout;
    let deadline = Instant::now() + config.duration;

    for _ in 0..config.virtual_users {
        let shared = shared.clone();

        tasks.push(tokio::spawn(async move {
            let mut results = Vec::new();

            while Instant::now() < deadline {
                let start = Instant::now();
                let result =
                    timeout(request_timeout, get_request(&shared.client, &shared.url)).await;
                let elapsed = start.elapsed();
                results.push((result.is_ok(), elapsed));
            }

            results
        }));
    }

    let mut success_count = 0;
    let mut error_count = 0;
    let mut latencies = Vec::with_capacity(config.virtual_users as usize);

    for task in tasks {
        match task.await {
            Ok(results) => {
                for (ok, elapsed) in results {
                    // Populate stats
                    if ok {
                        success_count += 1
                    } else {
                        error_count += 1
                    };

                    latencies.push(elapsed);
                }
            }
            Err(join_error) => {
                error_count += 1;
                println!("worker task failed: {join_error}");
            }
        }
    }

    Ok(RunMetrics {
        test_duration: run_start.elapsed(),
        success_count,
        error_count,
        latencies,
    })
}
