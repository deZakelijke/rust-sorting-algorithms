use clap::Parser;
use rust_sorting_algorithms::{Args, Config};
use std::process;

fn main() {
    let args = Args::parse();

    let config = Config::<i64>::new(&args).unwrap_or_else(|err| {
        println!("Incorrect configuration: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_sorting_algorithms::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
