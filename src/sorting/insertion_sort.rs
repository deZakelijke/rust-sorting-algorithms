pub fn insertion_sort<T: PartialOrd + Copy>(mut unsorted_numbers: Vec<T>) -> Vec<T> {
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
