use rand::{distributions::Uniform, Rng};
use std::error::Error;
use std::time::Instant;

pub struct Config {
    numbers_to_sort: u32,
    upper_limit: i32,
    lower_limit: i32,
    algorithms: Vec<String>,
}

impl Config {
    pub fn new(
        numbers_to_sort: u32,
        upper_limit: i32,
        lower_limit: i32,
        algorithms: Vec<String>,
    ) -> Result<Config, &'static str> {
        Ok(Config {
            numbers_to_sort,
            upper_limit,
            lower_limit,
            algorithms,
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
        println!("Sorting with '{}'", algorithm);
        let start = Instant::now();
        let elapsed_time = start.elapsed();
        println!("It took {:.3?} seconds", elapsed_time);
    }
    Ok(())
}

fn check_if_sorted(sorted_vector: &Vec<i32>) -> Result<(), ()> {
    let mut vec_iter = sorted_vector.iter().peekable();
    while let Some(item) = vec_iter.next() {
        if let Some(next) = vec_iter.peek() {
            if next < &item {
                return Err(());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_is_sorted() {
        let sorted_vector: Vec<i32> = (0..10).collect();
        assert_eq!(Ok(()), check_if_sorted(&sorted_vector));
    }

    #[test]
    fn simple_is_not_sorted() {
        let unsorted_vector: Vec<i32> = (0..10).rev().collect();
        assert_eq!(Err(()), check_if_sorted(&unsorted_vector));
    }
}