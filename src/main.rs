use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::mem::size_of_val;
use std::path::Path;

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
        .replace("-", "")
        .replace("\"", "");
    let words = contents.split_whitespace();

    let mut word_counts:HashMap<String, i32> = HashMap::new();

    for word in words {
        let word = word.to_lowercase();
        match word_counts.get(&word) {
            Some(count) => word_counts.insert(word.to_string(), count + 1),
            None => word_counts.insert(word.to_string(), 1)
        };
    }

    let mut vec = Vec::from_iter(word_counts);
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, count) in & vec {
    //    println!("{}={}", word, count);
    }
    let passive_words = get_passive();
    for word in & passive_words {
        let count = count_occurrences(word, &contents);
        if count > 0 {
            println!("{}={}", word, count);
        }
    }
    //println!("{:?}", passive_words);
}

fn get_passive() -> Vec<String> {
    let mut passive_words: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("passive.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                passive_words.push(word)
            }
        }
    }
    passive_words
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_occurrences(pattern: &String, body: &String) -> usize {
    let v = body.matches(pattern);
    v.count()
}

