use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Algorithm {
    BubbleSort(SortFn),
    InsertionSort(SortFn),
}

impl FromStr for Algorithm {
    type Err = ();

    fn from_str(input: &str) -> Result<Algorithm, Self::Err> {
        match input {
            "bubble_sort" => Ok(Algorithm::BubbleSort(bubble_sort)),
            _ => Err(()),
        }
    }
}
impl Algorithm {
    pub fn run_sorting_algorithm(&self, unsorted_numbers: Vec<i32>) -> Result<Vec<i32>, ()> {
        use Algorithm::*;

        match self {
            &BubbleSort(sort_fn) => Ok(sort_fn(unsorted_numbers)),
            _ => {
                println!("Sorting algorithm not yet implemented");
                Err(())
            }
        }
    }
}

type SortFn = fn(Vec<i32>) -> Vec<i32>;

pub fn check_if_sorted(sorted_vector: &Vec<i32>) -> Result<(), ()> {
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

fn bubble_sort(mut unsorted_numbers: Vec<i32>) -> Vec<i32> {
    let length = unsorted_numbers.len();
    for i in 0..length {
        for j in 1..length - i {
            let current_number = unsorted_numbers[j - 1];
            let next_number = unsorted_numbers[j];
            if current_number > next_number {
                unsorted_numbers.swap(j - 1, j);
            }
        }
    }
    unsorted_numbers
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
        let unsorted_numbers: Vec<i32> = (0..10).rev().collect();
        assert_eq!(Err(()), check_if_sorted(&unsorted_numbers));
    }
}
