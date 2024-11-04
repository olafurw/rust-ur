use crate::utils::word_minus_ur;

#[derive(Debug)]
pub struct IsWord {
    pub word: String,
    pub word_no_ur: String,
}
impl IsWord {
    pub fn new(word: &str) -> Self {
        let word_no_ur = word_minus_ur(word);
        Self {
            word: word.trim().to_string(),
            word_no_ur,
        }
    }
}

impl PartialEq<OtherWord> for IsWord {
    fn eq(&self, other: &OtherWord) -> bool {
        self.word_no_ur == other.word && self.word != self.word_no_ur
    }
}
impl PartialEq for IsWord {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}
impl Eq for IsWord {}

#[derive(Debug)]
pub struct OtherWord {
    pub word: String,
}
impl OtherWord {
    pub fn new(word: &str) -> Self {
        Self {
            word: word.trim().to_string(),
        }
    }
}

impl PartialEq<IsWord> for OtherWord {
    fn eq(&self, other: &IsWord) -> bool {
        other.word_no_ur == self.word && other.word != other.word_no_ur
    }
}
impl PartialEq for OtherWord {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}
impl Eq for OtherWord {}

#[cfg(test)]
mod word_struct_test {
    use super::*;

    #[test]
    fn test() {
        // equals
        assert_eq!(IsWord::new("hestur"), IsWord::new("hestur"));
        assert_eq!(OtherWord::new("hest"), OtherWord::new("hest"));
        assert_eq!(IsWord::new("hestur"), OtherWord::new("hest"));
        assert_eq!(OtherWord::new("hest"), IsWord::new("hestur"));

        // not equals
        assert_ne!(IsWord::new("hest"), OtherWord::new("hest"));
        assert_ne!(IsWord::new("hest"), OtherWord::new("hestur"));
    }
}
