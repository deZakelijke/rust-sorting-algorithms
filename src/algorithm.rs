pub use crate::sorting::*;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Algorithm {
    BubbleSort(SortFn),
    InsertionSort(SortFn),
    SelectionSort(SortFn),
    MergeSort(SortFn),
}

impl FromStr for Algorithm {
    type Err = ();

    fn from_str(input: &str) -> Result<Algorithm, Self::Err> {
        match input {
            "bubble_sort" => Ok(Algorithm::BubbleSort(bubble_sort)),
            "insertion_sort" => Ok(Algorithm::InsertionSort(insertion_sort)),
            "selection_sort" => Ok(Algorithm::SelectionSort(selection_sort)),
            "merge_sort" => Ok(Algorithm::MergeSort(merge_sort)),
            _ => Err(()),
        }
    }
}
impl Algorithm {
    pub fn run_sorting_algorithm(&self, unsorted_numbers: Vec<i32>) -> Result<Vec<i32>, ()> {
        use Algorithm::*;

        match self {
            &BubbleSort(sort_fn)
            | &InsertionSort(sort_fn)
            | &SelectionSort(sort_fn)
            | &MergeSort(sort_fn) => Ok(sort_fn(unsorted_numbers)),
        }
    }
}

type SortFn = fn(Vec<i32>) -> Vec<i32>;
