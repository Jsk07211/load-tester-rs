use crate::config::Config;
use crate::http::get_request;
use crate::metrics::RunMetrics;
use reqwest::{Client, Url};
use std::{
    sync::{Arc, atomic::Ordering},
    time::Instant,
};

#[derive(Clone)]
pub struct SharedState {
    pub client: Client,
    pub url: Url,
    pub metrics: Arc<RunMetrics>,
}

impl TryFrom<&Config> for SharedState {
    type Error = anyhow::Error; // Defines Self::Error

    fn try_from(config: &Config) -> anyhow::Result<Self> {
        Ok(SharedState {
            client: Client::new(),
            url: config.endpoint.clone(),
            metrics: Arc::new(RunMetrics::default()),
        })
    }
}

pub async fn worker_loop(
    config: &Config,
    shared: SharedState,
) -> anyhow::Result<SharedState, anyhow::Error> {
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
                    shared.metrics.success_count.fetch_add(1, Ordering::Relaxed);
                } else {
                    shared.metrics.error_count.fetch_add(1, Ordering::Relaxed);
                }

                shared.metrics.latencies.lock().unwrap().push(elapsed);
            }
        }));
    }

    for task in tasks {
        task.await?;
    }

    Ok(shared)
}
