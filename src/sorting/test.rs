#[cfg(test)]
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
fn bubble_sort_ten_items() {
    let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
    let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(sorted_numbers, bubble_sort::bubble_sort(unsorted_numbers));
}

#[test]
fn insertion_sort_ten_items() {
    let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
    let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(
        sorted_numbers,
        insertion_sort::insertion_sort(unsorted_numbers)
    );
}

#[test]
fn selection_sort_ten_items() {
    let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
    let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(
        sorted_numbers,
        selection_sort::selection_sort(unsorted_numbers)
    );
}

#[test]
fn merge_sort_ten_items() {
    let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
    let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(sorted_numbers, merge_sort::merge_sort(unsorted_numbers));
}

#[test]
fn quick_sort_ten_items() {
    let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
    let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(sorted_numbers, quick_sort::quick_sort(unsorted_numbers));
}

#[test]
fn radix_msd_sort_ten_items() {
    let unsorted_numbers: Vec<u8> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
    let sorted_numbers: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(sorted_numbers, radix_sort::radix_msd_sort(unsorted_numbers));
}

#[test]
fn heap_sort_ten_items() {
    let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
    let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(sorted_numbers, heap_sort::heap_sort(unsorted_numbers));
}
