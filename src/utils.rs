use std::collections::BTreeSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn words_to_vec(filename: &str) -> BTreeSet<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut words = BTreeSet::new();

    for line in reader.lines() {
        words.insert(line.unwrap());
    }

    return words;
}

pub fn word_minus_ur(is_word: &str) -> String {
    let ends_ur = is_word.ends_with("ur");
    if !ends_ur {
        return is_word.to_string();
    }

    let is_word = &is_word[..is_word.len() - 2];
    is_word.to_string()
}

pub fn extract_word(json_line: &String) -> String {
    let json_line = json::parse(json_line.as_str()).unwrap();
    let word = json_line["word"].as_str().unwrap();
    return word.to_string().to_lowercase();
}

pub fn word_is_alphabetic(word: &String) -> bool {
    for c in word.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    return true;
}

pub fn words_from_json(filename: &str) -> BTreeSet<String> {
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