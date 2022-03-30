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

pub fn bubble_sort(mut unsorted_numbers: Vec<i32>) -> Vec<i32> {
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

pub fn insertion_sort(mut unsorted_numbers: Vec<i32>) -> Vec<i32> {
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

pub fn selection_sort(mut unsorted_numbers: Vec<i32>) -> Vec<i32> {
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

pub fn merge_sort(mut unsorted_numbers: Vec<i32>) -> Vec<i32> {
    if unsorted_numbers.len() == 1 {
        return unsorted_numbers;
    }
    let half_unsorted = unsorted_numbers.split_off(unsorted_numbers.len() / 2);
    let first_half_sorted = merge_sort(unsorted_numbers);
    let second_half_sorted = merge_sort(half_unsorted);
    let mut first_half_sorted_iterable = first_half_sorted.iter().peekable();
    let mut second_half_sorted_iterable = second_half_sorted.iter().peekable();

    let mut sorted_numbers: Vec<i32> = Vec::new();
    while let (Some(first_head), Some(second_head)) = (
        first_half_sorted_iterable.peek(),
        second_half_sorted_iterable.peek(),
    ) {
        if first_head > second_head {
            let second_head = second_half_sorted_iterable.next().unwrap();
            sorted_numbers.push(*second_head);
        } else {
            let first_head = first_half_sorted_iterable.next().unwrap();
            sorted_numbers.push(*first_head);
        }
    }
    if let Some(_) = first_half_sorted_iterable.peek() {
        while let Some(first_head) = first_half_sorted_iterable.next() {
            sorted_numbers.push(*first_head);
        }
    } else {
        while let Some(second_head) = second_half_sorted_iterable.next() {
            sorted_numbers.push(*second_head);
        }
    }
    sorted_numbers
}

pub fn quick_sort(unsorted_numbers: Vec<i32>) -> Vec<i32> {
    if unsorted_numbers.len() <= 1 {
        return unsorted_numbers;
    }
    let pivot = unsorted_numbers[0];
    let mut low_half: Vec<i32> = Vec::new();
    let mut high_half: Vec<i32> = Vec::new();
    for number in unsorted_numbers {
        if number < pivot {
            low_half.push(number);
        } else if number > pivot {
            high_half.push(number);
        }
    }
    low_half = quick_sort(low_half);
    high_half = quick_sort(high_half);
    low_half.push(pivot);
    low_half.append(&mut high_half);
    low_half
}
pub fn radix_msb_sort(unsorted_numbers: Vec<i32>) -> Vec<i32> {
    radix_msb_sort_recursive(unsorted_numbers, 32)
}

fn radix_msb_sort_recursive(unsorted_numbers: Vec<i32>, bit: u8) -> Vec<i32> {
    if unsorted_numbers.len() <= 1 {
        return unsorted_numbers;
    }
    if bit == 0 {
        return unsorted_numbers;
    }

    let mut zero_half: Vec<i32> = Vec::new();
    let mut one_half: Vec<i32> = Vec::new();
    for number in unsorted_numbers {
        if number & (1 << bit - 1) == 0 {
            one_half.push(number);
        } else {
            zero_half.push(number);
        }
    }
    one_half = radix_msb_sort_recursive(one_half, bit - 1);
    zero_half = radix_msb_sort_recursive(zero_half, bit - 1);
    one_half.append(&mut zero_half);
    one_half
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
    fn bubble_sort_ten_items() {
        let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
        let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sorted_numbers, bubble_sort(unsorted_numbers));
    }

    #[test]
    fn insertion_sort_ten_items() {
        let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
        let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sorted_numbers, insertion_sort(unsorted_numbers));
    }

    #[test]
    fn selection_sort_ten_items() {
        let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
        let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sorted_numbers, selection_sort(unsorted_numbers));
    }

    #[test]
    fn merge_sort_ten_items() {
        let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
        let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sorted_numbers, merge_sort(unsorted_numbers));
    }

    #[test]
    fn quick_sort_ten_items() {
        let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
        let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sorted_numbers, quick_sort(unsorted_numbers));
    }

    #[test]
    fn radix_msb_sort_ten_items() {
        let unsorted_numbers: Vec<i32> = vec![8, 9, 1, 7, 2, 5, 3, 10, 4, 6];
        let sorted_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sorted_numbers, radix_msb_sort(unsorted_numbers));
    }
}
