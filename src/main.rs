use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
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
    let mut adverbs: Vec<(&String, &i32)> = Vec::new();
    let mut ings: Vec<(&String, &i32)> = Vec::new();

    let passive_words = get_passive();
    let mut passive_word_counts: Vec<(&String, usize)> = Vec::new();

    println!("Repeated Words");
    println!("--------------");
    let not_adverbs = get_not_adverbs();
    for (word, count) in & vec {
        if count > &1 {
            // if word.ends_with("ly") { //} && !not_adverbs.contains(word) {
            //     adverbs.push((word, count));
            // } else if word.ends_with("ing") {
            //     ings.push((word, count));
            // } else {
                println!("{}={}", word, count);
            // }
        }
    //
    }

    println!();
    println!("Adverbs ({})", adverbs.len());
    println!("------------");
    adverbs.sort_by(|a, b| b.1.cmp(&a.1));
    for (adverb, count) in adverbs {
        println!("{}={}", adverb, count);
    }

    println!();
    println!("-ings ({})", ings.len());
    println!("----------");
    ings.sort_by(|a, b| b.1.cmp(&a.1));
    for (ing, count) in ings {
        println!("{}={}", ing, count);
    }

    for word in & passive_words {
        let count = count_occurrences(word, &contents);
        if count > 0 {
            passive_word_counts.push((word, count));
        }
    }
    passive_word_counts.sort_by(|a, b| b.1.cmp(&a.1));

    println!();
    println!("Passive Words/Phrases");
    println!("---------------------");
    for tuple in passive_word_counts {
        println!("{}={}", tuple.0, tuple.1);
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

fn get_not_adverbs() -> Vec<String> {
    let mut not_adverbs: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("not_adverbs.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                not_adverbs.push(word)
            }
        }
    }
    not_adverbs
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

