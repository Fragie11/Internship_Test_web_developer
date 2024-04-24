// Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first_string = &strings[0];
    let mut longest_prefix = String::new();
    'outer: for (i, c) in first_string.chars().enumerate() {
        for string in strings.iter().skip(1) {
            if let Some(sc) = string.chars().nth(i) {
                if sc != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        longest_prefix.push(c);
    }
    longest_prefix
}

fn main() {
    let strings = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let longest_prefix = longest_common_prefix(&strings);
    println!("Longest common prefix is: {}", longest_prefix);
}
