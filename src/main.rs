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
    let parts = contents.split_whitespace();

    for word in parts {
        println!("{word}");
    }
}
