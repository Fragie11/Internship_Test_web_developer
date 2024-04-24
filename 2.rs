// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    let sorted_array = [1, 3, 5, 7, 9, 11];
    let target_number = 7;
    match first_occurrence(&sorted_array, target_number) {
        Some(index) => println!("First occurrence of {} is at index {}.", target_number, index),
        None => println!("{} is not found in the array.", target_number),
    }
}
