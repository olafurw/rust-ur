use std::collections::BTreeSet;

use crate::utils::word_is_alphabetic;

pub fn extract_word(json_line: &str) -> Option<String> {
    let json_line = json::parse(json_line);
    if json_line.is_err() {
        return None;
    }

    let json_line = json_line.unwrap();
    let word = json_line["word"].as_str();

    Some(word?.to_string().to_lowercase())
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

    words
}

#[cfg(test)]
mod words_from_jsonl_test {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn test() {
        let jsonl = vec![
            String::from("{ \"word\": \"hello\"}"),
            String::from("{ \"word\": \"world\"}"),
            String::from("{ \"word\": \"\"}"),
            String::from("{ \"xx\": \"hello\"}"),
            String::from("{ \"word\": \"world\"}"),
        ];

        let words = words_from_jsonl(&jsonl);

        let mut btree_words = BTreeSet::new();
        btree_words.insert(String::from("hello"));
        btree_words.insert(String::from("world"));
        btree_words.insert(String::from(""));
        assert_eq!(words, btree_words);
    }
}
