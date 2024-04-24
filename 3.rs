// Given a string of words, implement a function that returns the shortest word in the string.

fn shortest_word(sentence: &str) -> &str {
    sentence.split_whitespace().min_by_key(|word| word.len()).unwrap_or("")
}

fn main() {
    let sentence = "This is a test sentence.";
    let shortest = shortest_word(sentence);
    println!("The shortest word in the sentence is: {}", shortest);
}
