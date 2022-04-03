pub fn heap_sort<T: PartialOrd + Copy>(mut unsorted_numbers: Vec<T>) -> Vec<T> {
    heapify(&mut unsorted_numbers);
    for end in (1..(unsorted_numbers.len())).rev() {
        unsorted_numbers.swap(0, end);
        sift_down(&mut unsorted_numbers, 0, end - 1);
    }
    unsorted_numbers
}
fn heapify<T: PartialOrd>(numbers: &mut Vec<T>) {
    for start in (0..((numbers.len() - 2) / 2 + 1)).rev() {
        sift_down(numbers, start, numbers.len() - 1);
    }
}
fn sift_down<T: PartialOrd>(numbers: &mut Vec<T>, start_index: usize, end_index: usize) {
    let mut root = start_index;

    while root * 2 + 1 <= end_index {
        let child = root * 2 + 1;
        let mut swap = root;

        if numbers[swap] < numbers[child] {
            swap = child;
        }
        if child + 1 <= end_index && numbers[swap] < numbers[child + 1] {
            swap = child + 1;
        }

        if swap == root {
            return;
        } else {
            numbers.swap(root, swap);
            root = swap;
        }
    }
}
