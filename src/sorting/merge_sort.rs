pub fn merge_sort<T: PartialOrd + Copy>(mut unsorted_numbers: Vec<T>) -> Vec<T> {
    if unsorted_numbers.len() == 1 {
        return unsorted_numbers;
    }
    let half_unsorted = unsorted_numbers.split_off(unsorted_numbers.len() / 2);
    let first_half_sorted = merge_sort(unsorted_numbers);
    let second_half_sorted = merge_sort(half_unsorted);
    let mut first_half_sorted_iterable = first_half_sorted.iter().peekable();
    let mut second_half_sorted_iterable = second_half_sorted.iter().peekable();

    let mut sorted_numbers: Vec<T> = Vec::new();
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
