use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Algorithm {
    BubbleSort(SortFn),
    InsertionSort(SortFn),
    SelectionSort(SortFn),
}

impl FromStr for Algorithm {
    type Err = ();

    fn from_str(input: &str) -> Result<Algorithm, Self::Err> {
        match input {
            "bubble_sort" => Ok(Algorithm::BubbleSort(bubble_sort)),
            "insertion_sort" => Ok(Algorithm::InsertionSort(insertion_sort)),
            "selection_sort" => Ok(Algorithm::SelectionSort(selection_sort)),
            _ => Err(()),
        }
    }
}
impl Algorithm {
    pub fn run_sorting_algorithm(&self, unsorted_numbers: Vec<i32>) -> Result<Vec<i32>, ()> {
        use Algorithm::*;

        match self {
            &BubbleSort(sort_fn) | &InsertionSort(sort_fn) | &SelectionSort(sort_fn) => {
                Ok(sort_fn(unsorted_numbers))
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

fn insertion_sort(mut unsorted_numbers: Vec<i32>) -> Vec<i32> {
    let length = unsorted_numbers.len();
    for i in 0..length {
        let current_number = unsorted_numbers[i];
        for j in 0..i {
            let current_smallest = unsorted_numbers[j];
            if current_number <= current_smallest {
                unsorted_numbers.remove(i);
                unsorted_numbers.insert(j, current_number);
                break;
            }
        }
    }
    unsorted_numbers
}

fn selection_sort(mut unsorted_numbers: Vec<i32>) -> Vec<i32> {
    let length = unsorted_numbers.len();
    for i in 0..length {
        let mut current_smallest = unsorted_numbers[i];
        let mut current_smallest_index = i;
        for j in i..length {
            if unsorted_numbers[j] < current_smallest {
                current_smallest = unsorted_numbers[j];
                current_smallest_index = j;
            }
        }
        unsorted_numbers.swap(i, current_smallest_index);
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

    #[test]
    fn bubble_sort_five_items() {
        let unsorted_numbers: Vec<i32> = vec![4, 2, 5, 3, 1];
        let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(sorted_numbers, bubble_sort(unsorted_numbers));
    }

    #[test]
    fn insertion_sort_five_items() {
        let unsorted_numbers: Vec<i32> = vec![4, 2, 5, 3, 1];
        let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(sorted_numbers, insertion_sort(unsorted_numbers));
    }

    #[test]
    fn selection_sort_five_items() {
        let unsorted_numbers: Vec<i32> = vec![4, 2, 5, 3, 1];
        let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(sorted_numbers, selection_sort(unsorted_numbers));
    }
}
