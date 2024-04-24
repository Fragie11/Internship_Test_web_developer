// Implement a function that returns the kth smallest element in a given array.

fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).cloned()
}

fn main() {
    let arr = vec![4, 2, 1, 6, 5, 3];
    let k = 3;
    match kth_smallest_element(&arr, k) {
        Some(smallest) => println!("The {}rd smallest element is: {}", k, smallest),
        None => println!("Array is too small."),
    }
}
