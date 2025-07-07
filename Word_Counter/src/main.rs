use std::env;
use std::fs::File;
use std::io::{Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <filename>");
        return;
    }

    let file_path = &args[1];

    println!("Reading file: {}", file_path);

   // Read the file contents
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening file: {}", error);
            return;
        }

    };

    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", error);
        return;
    }
    // Count Words
    let word_count = count_words(&contents);
    println!("Word Count: {}", word_count);
            
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}