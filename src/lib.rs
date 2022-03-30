use clap::Parser;
use rand::{distributions::Uniform, Rng};
use std::error::Error;
use std::str::FromStr;
use std::time::Instant;

mod algorithm;
mod sorting;
pub use crate::algorithm::Algorithm;
pub use crate::sorting::check_if_sorted;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // Amount of numbers to sort
    #[clap(short, long, default_value_t = 10000)]
    numbers: u32,

    // Maximum value in unsorted numbers
    #[clap(long, default_value_t = 100000)]
    max: i32,

    // Minimum value in unsorted numbers
    #[clap(long, default_value_t = 0)]
    min: i32,

    // Sorting algorithms to use
    #[clap(short, long)]
    algorithms: Vec<String>,
}
pub struct Config {
    numbers: u32,
    max: i32,
    min: i32,
    algorithms: Vec<Algorithm>,
}

impl Config {
    pub fn new(args: &Args) -> Result<Config, &'static str> {
        let mut valid_algorithms: Vec<Algorithm> = Vec::new();

        for algorithm in args.algorithms.iter() {
            let algorithm = Algorithm::from_str(&algorithm);
            match algorithm {
                Ok(algorithm) => valid_algorithms.push(algorithm),
                Err(error) => return Err(error),
            }
        }

        Ok(Config {
            numbers: args.numbers,
            max: args.max,
            min: args.min,
            algorithms: valid_algorithms,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(config.min, config.max);
    let random_numbers: Vec<i32> = (0..config.numbers).map(|_| rng.sample(&range)).collect();

    println!("Sorting {} numbers", config.numbers);
    for algorithm in config.algorithms {
        println!("Sorting with '{:?}'", algorithm);
        let start = Instant::now();
        let sorted_numbers = algorithm
            .run_sorting_algorithm(random_numbers.clone())
            .unwrap();

        let elapsed_time = start.elapsed();
        println!("It took {:.3?} seconds", elapsed_time);
        let sort_check = check_if_sorted(&sorted_numbers);
        match sort_check {
            Ok(()) => println!("Sorted correctly"),
            Err(()) => println!("Not sorted correctly"),
        };
    }
    Ok(())
}
