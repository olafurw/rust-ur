use std::collections::BTreeSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn words_to_vec(filename: &str) -> BTreeSet<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut words = BTreeSet::new();

    for line in reader.lines() {
        words.insert(line.unwrap());
    }

    return words;
}

fn main() {
    let is_words = words_to_vec("is_words.txt");
    let se_words = words_to_vec("se_words.txt");
    
    for word in is_words {
        if se_words.contains(&word) {
            println!("{}", word);
        }
    }
}