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

    return lines;
}

pub fn words_to_btree_set(words: &Vec<String>) -> BTreeSet<String> {
    let mut btree_words = BTreeSet::new();

    for word in words {
        btree_words.insert(word.clone());
    }

    btree_words
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

pub fn extract_word(json_line: &String) -> Option<String> {
    let json_line = json::parse(json_line.as_str());
    if json_line.is_err() {
        return None;
    }

    let json_line = json_line.unwrap();
    let word = json_line["word"].as_str();
    if word.is_none() {
        return None;
    }

    return Some(word.unwrap().to_string().to_lowercase());
}

#[cfg(test)]
mod extract_word_test {
    use super::*;

    #[test]
    fn test() {
        let extract_a = extract_word(&String::from("{ \"word\": \"hello\"}"));
        assert_eq!(extract_a, Some(String::from("hello")));

        let extract_b = extract_word(&String::from("{ \"word\": \"\"}"));
        assert_eq!(extract_b, Some(String::from("")));

        let extract_c = extract_word(&String::from("{ \"xx\": \"hello\"}"));
        assert_eq!(extract_c, None);
    }
}

pub fn word_is_alphabetic(word: &String) -> bool {
    for c in word.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod word_is_alphabetic_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(word_is_alphabetic(&String::from("hello")), true);
        assert_eq!(word_is_alphabetic(&String::from("")), true);
        assert_eq!(word_is_alphabetic(&String::from("    ")), false);

        assert_eq!(word_is_alphabetic(&String::from("123")), false);
        assert_eq!(word_is_alphabetic(&String::from("  123  ")), false);
        assert_eq!(word_is_alphabetic(&String::from("!@#!@#")), false);
    }
}

pub fn words_from_jsonl(jsonl: &Vec<String>) -> BTreeSet<String> {
    let mut words = BTreeSet::new();

    for line in jsonl {
        let word = extract_word(line);
        if word.is_none() {
            continue;
        }

        let word = word.unwrap();
        let is_valid = word_is_alphabetic(&word);
        if !is_valid {
            continue;
        }
        
        words.insert(word);
    }

    return words;
}

pub fn normalize(text: &str) -> String {
    let sktext = deunicode(text);
    sktext
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