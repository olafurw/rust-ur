use utils::{file_to_vec, words_from_jsonl};

mod utils;

fn main() {
    let line = file_to_vec("danish.jsonl");
    let words = words_from_jsonl(&line);
    println!("{:?}", words);
}