use core::ops::{BitAnd, Shl};
use num::{PrimInt, Zero};

pub fn radix_msd_sort<T>(unsorted_numbers: Vec<T>) -> Vec<T>
where
    T: PartialOrd + PartialEq + PrimInt + Zero + BitAnd<i64, Output = T>,
{
    radix_msd_sort_recursive(unsorted_numbers, 32)
}

fn radix_msd_sort_recursive<T>(unsorted_numbers: Vec<T>, bit: u8) -> Vec<T>
where
    T: PartialOrd + PartialEq + PrimInt + Zero + BitAnd<i64, Output = T>,
{
    if unsorted_numbers.len() <= 1 {
        return unsorted_numbers;
    }
    if bit == 0 {
        return unsorted_numbers;
    }

    let mut zero_half: Vec<T> = Vec::new();
    let mut one_half: Vec<T> = Vec::new();

    let mask: i64 = 1 << bit - 1;

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
