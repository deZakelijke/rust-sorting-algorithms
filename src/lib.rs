use clap::Parser;
use core::fmt::Display;
use core::ops::BitAnd;
use num::Integer;
use num::{PrimInt, Zero};
use rand::distributions::{uniform::SampleUniform, Uniform};
use rand::Rng;
use std::error::Error;
use std::ops::{Range, Rem};
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
    numbers: i64,

    // Maximum value in unsorted numbers
    #[clap(long, default_value_t = 100000)]
    max: i64,

    // Minimum value in unsorted numbers
    #[clap(long, default_value_t = 0)]
    min: i64,

    // Sorting algorithms to use
    #[clap(short, long)]
    algorithms: Vec<String>,
}
pub struct Config<T> {
    numbers: T,
    max: T,
    min: T,
    algorithms: Vec<Algorithm<T>>,
}

impl<T> Config<T>
where
    T: PartialEq
        + PartialOrd
        + Copy
        + PrimInt
        + Zero
        + BitAnd<i64, Output = T>
        + Rem<Output = T>
        + From<u8>
        + From<i64>,
{
    pub fn new(args: &Args) -> Result<Config<T>, &'static str> {
        let mut valid_algorithms: Vec<Algorithm<T>> = Vec::new();
        let algorithms = vec!["merge_sort", "heap_sort", "quick_sort", "radix_msd_sort"];
        for algorithm in algorithms {
            let algorithm = Algorithm::from_str(algorithm);
            match algorithm {
                Ok(algorithm) => valid_algorithms.push(algorithm),
                Err(error) => return Err(error),
            }
        }
        Ok(Config {
            numbers: args.numbers.try_into().unwrap(),
            max: args.max.try_into().unwrap(),
            min: args.min.try_into().unwrap(),
            algorithms: valid_algorithms,
        })
    }
}

pub fn run<T>(config: Config<T>) -> Result<(), Box<dyn Error>>
where
    T: PartialEq
        + PartialOrd
        + Copy
        + Clone
        + Display
        + Integer
        + PrimInt
        + Zero
        + Rem<Output = T>
        + SampleUniform
        + BitAnd<i64, Output = T>,
    Range<T>: Iterator<Item = T>,
    Vec<T>: FromIterator<T>,
{
    let mut rng = rand::thread_rng();
    let range = Uniform::new(config.min, config.max);
    let random_numbers: Vec<T> = (T::zero()..config.numbers)
        .map(|_| rng.sample(&range))
        .collect();

    println!("Sorting {} numbers", config.numbers);
    for algorithm in config.algorithms {
        println!("Sorting with '{}'", algorithm);
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
