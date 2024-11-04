use utils::{extract_is_ur_words, extract_other_words};
mod jsonl;
mod utils;
mod word;

fn main() {
    let is_words = extract_is_ur_words("dictionaries/is.dict");
    let nb_words = extract_other_words("dictionaries/nb.dict");

    is_words.intersection(&nb_words).for_each(|word| {
        println!("{}", word);
    });
}
