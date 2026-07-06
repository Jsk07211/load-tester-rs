use crate::config::Config;
use crate::metrics::get_summary;
use crate::worker::{self, SharedState};

pub async fn run(config: Config) -> anyhow::Result<()> {
    let initial_state = SharedState::try_from(&config)?;
    let shared = worker::worker_loop(&config, initial_state).await?;
    let summary = get_summary(&shared.metrics, config.duration);
    print!("{}", summary.report());

    Ok(())
}
