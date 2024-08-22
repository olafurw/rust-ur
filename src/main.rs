use deunicode::deunicode;
use utils::{word_minus_ur, words_to_vec};

mod utils;

fn main() {
    let text = "åfløyer";
    let sktext = deunicode(text);
    
    println!("Original: {}", text);
    println!("NFC: {}", sktext);
}