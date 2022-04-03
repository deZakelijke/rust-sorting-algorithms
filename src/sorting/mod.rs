pub mod bubble_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod radix_sort;
pub mod selection_sort;
mod test;

pub fn check_if_sorted<T>(sorted_vector: &Vec<T>) -> Result<(), ()>
where
    T: PartialOrd + PartialEq,
{
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
