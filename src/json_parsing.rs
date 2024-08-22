use std::collections::BTreeSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn extract_word(json_line: &String) -> String {
    let json_line = json::parse(json_line.as_str()).unwrap();
    let word = json_line["word"].as_str().unwrap();
    return word.to_string().to_lowercase();
}

fn word_is_alphabetic(word: &String) -> bool {
    for c in word.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    return true;
}

fn words_to_vec(filename: &str) -> BTreeSet<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut words = BTreeSet::new();

    for line in reader.lines() {
        let word = extract_word(&line.unwrap());
        let is_valid = word_is_alphabetic(&word);
        if !is_valid {
            continue;
        }
        
        words.insert(word);
    }

    return words;
}

fn main() {
    let is_words = words_to_vec("swedish.jsonl");
    for word in is_words {
        println!("{}", word);
    }
    //let se_words = words_to_vec("swedish.jsonl");
    
    /*for word in is_words {
        if se_words.contains(&word) {
            println!("{}", word);
        }
    }*/
}