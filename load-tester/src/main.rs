use clap::Parser;
use load_tester::cmd::execute::Args;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
