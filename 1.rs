// Given a string, implement a function that checks whether it is a palindrome or not.

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn main() {
    let test_string = "racecar";
    if is_palindrome(test_string) {
        println!("{} is a palindrome.", test_string);
    } else {
        println!("{} is not a palindrome.", test_string);
    }
}
