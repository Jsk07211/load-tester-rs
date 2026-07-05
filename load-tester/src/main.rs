use clap::Parser;
use load_tester::cmd::{config, execute};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = execute::Args::parse();
    let config = config::Config::try_from(args)?;
    execute::run(config).await
}
