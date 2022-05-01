use core::ops::BitAnd;
use num::{PrimInt, Zero};
use std::mem::size_of;

pub fn radix_msd_sort<T>(unsorted_numbers: Vec<T>) -> Vec<T>
where
    T: PartialOrd + PartialEq + PrimInt + Zero + BitAnd<Output = T> + From<u8>,
{
    let n = size_of::<T>();
    println!("Size of T: {}", n);
    radix_msd_sort_recursive(unsorted_numbers, n * 8)
}

fn radix_msd_sort_recursive<T>(unsorted_numbers: Vec<T>, bit: usize) -> Vec<T>
where
    T: PartialOrd + PartialEq + PrimInt + Zero + BitAnd<Output = T> + From<u8>,
{
    if unsorted_numbers.len() <= 1 {
        return unsorted_numbers;
    }
    if bit == 0 {
        return unsorted_numbers;
    }

    let mut zero_half: Vec<T> = Vec::new();
    let mut one_half: Vec<T> = Vec::new();

    let mask = 1 << (bit - 1);
    let mask = mask.try_into().unwrap();

    for number in unsorted_numbers {
        if (number & mask).is_zero() {
            one_half.push(number);
        } else {
            zero_half.push(number);
        }
    }
    one_half = radix_msd_sort_recursive(one_half, bit - 1);
    zero_half = radix_msd_sort_recursive(zero_half, bit - 1);
    one_half.append(&mut zero_half);
    one_half
}
