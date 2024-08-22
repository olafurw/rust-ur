use std::fs::File;
use std::io::{prelude::*, BufReader};

fn extract_word(json_line: &String) -> String {
    let json_line = json::parse(json_line.as_str()).unwrap();
    let word = json_line["word"].as_str().unwrap();
    return word.to_string();
}

fn main() {
    let file = File::open("icelandic.jsonl").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = extract_word(&line.unwrap());
        let contains_space = word.contains(char::is_whitespace);
        if contains_space {
            continue;
        }
        
        let lower_word = word.to_lowercase();
        println!("{}, {}", word, lower_word);
    }
}