use rand::{distributions::Uniform, Rng};
use std::error::Error;
use std::str::FromStr;
use std::time::Instant;

mod sorting;
pub use crate::sorting::{check_if_sorted, Algorithm};

pub struct Config {
    numbers_to_sort: u32,
    upper_limit: i32,
    lower_limit: i32,
    algorithms: Vec<Algorithm>,
}

impl Config {
    pub fn new(
        numbers_to_sort: u32,
        upper_limit: i32,
        lower_limit: i32,
        algorithms: Vec<String>,
    ) -> Result<Config, &'static str> {
        let mut valid_algorithms: Vec<Algorithm> = Vec::new();

        for algorithm in algorithms {
            if let Ok(algorithm) = Algorithm::from_str(&algorithm) {
                valid_algorithms.push(algorithm);
            }
        }

        Ok(Config {
            numbers_to_sort,
            upper_limit,
            lower_limit,
            algorithms: valid_algorithms,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(config.lower_limit, config.upper_limit);
    let random_numbers: Vec<i32> = (0..config.numbers_to_sort)
        .map(|_| rng.sample(&range))
        .collect();

    println!("Sorting {} numbers", config.numbers_to_sort);
    println!("{:?}", random_numbers);
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
