use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let contents = contents.replace(".", "")
        .replace(",", "")
        .replace("!", "")
        .replace("?", "")
        .replace("\"", "");
    let words = contents.split_whitespace();

    let mut word_counts:HashMap<String, i32> = HashMap::new();

    for word in words {
        match word_counts.get(word) {
            Some(count) => word_counts.insert(word.to_string(), count + 1),
            None => word_counts.insert(word.to_string(), 1)
        };
    }

    let mut vec = Vec::from_iter(word_counts);
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, count) in & vec {
        println!("{}={}", word, count);
    }
}
