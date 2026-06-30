use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target URL to load test
    #[arg(short, long, default_value = "http://localhost:8080/ping")]
    pub endpoint: String,

    /// Number of concurrent users
    #[arg(short, long, default_value_t = 5)]
    pub users: u32,

    /// Duration of test
    #[arg(short, long, default_value_t = 30)]
    pub duration_s: u64,

    /// HTTP method
    #[arg(short, long, default_value = "GET")]
    pub method: String,
}
