pub fn quick_sort<T: PartialOrd + Copy>(unsorted_numbers: Vec<T>) -> Vec<T> {
    if unsorted_numbers.len() <= 1 {
        return unsorted_numbers;
    }
    let pivot = unsorted_numbers[0];
    let mut low_half: Vec<T> = Vec::new();
    let mut high_half: Vec<T> = Vec::new();
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
