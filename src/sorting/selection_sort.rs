pub fn selection_sort<T: PartialOrd + Copy>(mut unsorted_numbers: Vec<T>) -> Vec<T> {
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
