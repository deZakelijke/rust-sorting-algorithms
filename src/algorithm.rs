pub use crate::sorting::*;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Algorithm {
    algorithm_func: SortFn,
    algorithm_name: String,
}
// pub enum Algorithm {
//     BubbleSort(SortFn),
//     InsertionSort(SortFn),
//     SelectionSort(SortFn),
//     MergeSort(SortFn),
//     QuickSort(SortFn),
//     RadixMSBSort(SortFn),
// }

impl FromStr for Algorithm {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Algorithm, Self::Err> {
        match input {
            "bubble_sort" => Ok(Algorithm {
                algorithm_func: bubble_sort,
                algorithm_name: "Bubble sort".into(),
            }),
            "insertion_sort" => Ok(Algorithm {
                algorithm_func: insertion_sort,
                algorithm_name: "Insertion sort".into(),
            }),
            "selection_sort" => Ok(Algorithm {
                algorithm_func: selection_sort,
                algorithm_name: "Selection sort".into(),
            }),
            "merge_sort" => Ok(Algorithm {
                algorithm_func: merge_sort,
                algorithm_name: "Merge sort".into(),
            }),
            "quick_sort" => Ok(Algorithm {
                algorithm_func: quick_sort,
                algorithm_name: "Quick sort".into(),
            }),
            "radix_msb_sort" => Ok(Algorithm {
                algorithm_func: radix_msb_sort,
                algorithm_name: "Radix most significant bit sort".into(),
            }),
            "heap_sort" => Ok(Algorithm {
                algorithm_func: heap_sort,
                algorithm_name: "Heap sort".into(),
            }),
            _ => Err("Not a valid sorting algorithm"),
        }
    }
}
impl Algorithm {
    pub fn run_sorting_algorithm(&self, unsorted_numbers: Vec<i32>) -> Result<Vec<i32>, ()> {
        Ok((self.algorithm_func)(unsorted_numbers))
    }
}

type SortFn = fn(Vec<i32>) -> Vec<i32>;
