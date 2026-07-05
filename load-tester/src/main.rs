use clap::Parser;
use load_tester::{cmd::execute, config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = config::Args::parse();
    let config = config::Config::try_from(args)?;
    execute::run(config).await
}
