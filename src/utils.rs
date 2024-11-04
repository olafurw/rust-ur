use std::collections::BTreeSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use deunicode::deunicode;

pub fn file_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename);
    if file.is_err() {
        return Vec::new();
    }

    let file = file.unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        if line.is_err() {
            continue;
        }
        lines.push(line.unwrap());
    }

    lines
}

pub fn extract_is_ur_words(filename: &str) -> BTreeSet<String> {
    let file = File::open(filename);
    if file.is_err() {
        return BTreeSet::new();
    }

    let file = file.unwrap();
    let reader = BufReader::new(file);

    let mut btree_words = BTreeSet::new();
    for line in reader.lines() {
        if line.is_err() {
            continue;
        }

        let word = line.unwrap();
        let word_no_ur = word_minus_ur(&word);

        if word_no_ur != word {
            btree_words.insert(word_no_ur);
        }
    }

    btree_words
}

pub fn extract_other_words(filename: &str) -> BTreeSet<String> {
    let file = File::open(filename);
    if file.is_err() {
        return BTreeSet::new();
    }

    let file = file.unwrap();
    let reader = BufReader::new(file);

    let mut btree_words = BTreeSet::new();
    for line in reader.lines() {
        if line.is_err() {
            continue;
        }

        btree_words.insert(line.unwrap());
    }

    btree_words
}

pub fn words_to_btree_set(words: &Vec<String>) -> BTreeSet<String> {
    let mut btree_words = BTreeSet::new();

    for word in words {
        btree_words.insert(word.clone());
    }

    btree_words
}

#[cfg(test)]
mod words_to_btree_set_test {
    use super::*;

    #[test]
    fn test() {
        {
            let words = vec![String::from("hello"), String::from("world")];
            let mut btree_words = BTreeSet::new();
            btree_words.insert(String::from("hello"));
            btree_words.insert(String::from("world"));

            assert_eq!(words_to_btree_set(&words), btree_words);
        }
        {
            let words = vec![String::from("hello"), String::from("hello")];
            let mut btree_words = BTreeSet::new();
            btree_words.insert(String::from("hello"));

            assert_eq!(words_to_btree_set(&words), btree_words);
        }
        {
            let words = vec![
                String::from("hello"),
                String::from("world"),
                String::from("hello"),
            ];
            let mut btree_words = BTreeSet::new();
            btree_words.insert(String::from("hello"));
            btree_words.insert(String::from("world"));

            assert_eq!(words_to_btree_set(&words), btree_words);
        }
    }
}

pub fn word_minus_ur(is_word: &str) -> String {
    let trimmed_word = is_word.trim();
    let ends_ur = trimmed_word.ends_with("ur");
    if !ends_ur {
        return trimmed_word.to_string();
    }

    let trimmed_word = &trimmed_word[..trimmed_word.len() - 2];
    trimmed_word.to_string()
}

#[cfg(test)]
mod word_minus_ur_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(word_minus_ur("hestur"), String::from("hest"));
        assert_eq!(word_minus_ur("  hestur  "), String::from("hest"));
        assert_eq!(word_minus_ur(""), String::from(""));
        assert_eq!(word_minus_ur("hest"), String::from("hest"));
    }
}

pub fn word_is_alphabetic(word: &str) -> bool {
    for c in word.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod word_is_alphabetic_test {
    use super::*;

    #[test]
    fn test() {
        assert!(word_is_alphabetic("hello"));
        assert!(word_is_alphabetic(""));
        assert!(!word_is_alphabetic("    "));

        assert!(!word_is_alphabetic("123"));
        assert!(!word_is_alphabetic("  123  "));
        assert!(!word_is_alphabetic("!@#!@#"));
    }
}

pub fn normalize(text: &str) -> String {
    deunicode(text)
}

#[cfg(test)]
mod normalize_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(normalize("hello"), String::from("hello"));
        assert_eq!(normalize(""), String::from(""));
        assert_eq!(normalize("åfløyer"), String::from("afloyer"));
    }
}
