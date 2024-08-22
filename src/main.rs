use std::collections::BTreeSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
mod json_parsing;

fn words_to_vec(filename: &str) -> BTreeSet<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut words = BTreeSet::new();

    for line in reader.lines() {
        words.insert(line.unwrap());
    }

    return words;
}

fn word_minus_ur(is_word: &str) -> String {
    let ends_ur = is_word.ends_with("ur");
    if !ends_ur {
        return is_word.to_string();
    }

    let is_word = &is_word[..is_word.len() - 2];
    is_word.to_string()
}

fn main() {
    let is_words = words_to_vec("is.words");
    let other_words = words_to_vec("nb.words");
    
    for word in is_words {
        let word_no_ur = word_minus_ur(&word);
        if other_words.contains(&word) || other_words.contains(&word_no_ur) {
            println!("{}", word);
        }
    }
}