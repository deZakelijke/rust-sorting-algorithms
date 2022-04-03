pub use crate::sorting::*;
use core::ops::BitAnd;
use num::{PrimInt, Zero};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Algorithm<T> {
    algorithm_func: SortFn<T>,
    algorithm_name: String,
}

impl<T> FromStr for Algorithm<T>
where
    T: PartialEq + PartialOrd + Copy + PrimInt + Zero + BitAnd<i64, Output = T>,
{
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Algorithm<T>, Self::Err> {
        match input {
            "bubble_sort" => Ok(Algorithm {
                algorithm_func: bubble_sort::bubble_sort,
                algorithm_name: "Bubble sort".into(),
            }),
            "insertion_sort" => Ok(Algorithm {
                algorithm_func: insertion_sort::insertion_sort,
                algorithm_name: "Insertion sort".into(),
            }),
            "selection_sort" => Ok(Algorithm {
                algorithm_func: selection_sort::selection_sort,
                algorithm_name: "Selection sort".into(),
            }),
            "merge_sort" => Ok(Algorithm {
                algorithm_func: merge_sort::merge_sort,
                algorithm_name: "Merge sort".into(),
            }),
            "quick_sort" => Ok(Algorithm {
                algorithm_func: quick_sort::quick_sort,
                algorithm_name: "Quick sort".into(),
            }),
            "radix_msd_sort" => Ok(Algorithm {
                algorithm_func: radix_sort::radix_msd_sort,
                algorithm_name: "Radix most significant bit sort".into(),
            }),
            "heap_sort" => Ok(Algorithm {
                algorithm_func: heap_sort::heap_sort,
                algorithm_name: "Heap sort".into(),
            }),
            _ => Err("Not a valid sorting algorithm"),
        }
    }
}
impl<T> fmt::Display for Algorithm<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.algorithm_name)
    }
}
impl<T> Algorithm<T> {
    pub fn run_sorting_algorithm(&self, unsorted_numbers: Vec<T>) -> Result<Vec<T>, ()> {
        Ok((self.algorithm_func)(unsorted_numbers))
    }
}

type SortFn<T> = fn(Vec<T>) -> Vec<T>;
