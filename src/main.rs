use rust_sorting_algorithms::Config;
use std::process;

fn main() {
    let numbers_to_sort: u32 = 10;
    let upper_limit: i32 = 1000;
    let lower_limit: i32 = 0;

    let algorithms = vec!["bubble_sort".into()];
    let config = Config::new(numbers_to_sort, upper_limit, lower_limit, algorithms).unwrap();

    if let Err(e) = rust_sorting_algorithms::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
