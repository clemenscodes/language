pub(crate) mod error;

pub(crate) use error::Error;

use crate::core::language::vocabulary::word::Word;
use ddd::{DomainLayer, Identifier, Layered, ValueObject};
use std::fmt;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Spelling {
    words: Vec<Word>,
}

impl Layered for Spelling {
    type Layer = DomainLayer;
}

impl ValueObject for Spelling {}

impl Identifier for Spelling {}

impl TryFrom<Vec<Word>> for Spelling {
    type Error = Error;

    fn try_from(words: Vec<Word>) -> Result<Self, Self::Error> {
        if words.is_empty() {
            return Err(Error::Term);
        }
        Ok(Self { words })
    }
}

impl<'spelling> IntoIterator for &'spelling Spelling {
    type Item = &'spelling Word;
    type IntoIter = Iter<'spelling, Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.iter()
    }
}

impl fmt::Display for Spelling {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word_texts: Vec<String> = self.words.iter().map(Word::to_string).collect();
        let line = word_texts.join(" ");
        fmt::Display::fmt(&line, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_spelling_keeps_the_words_it_takes() {
        let king = Word::try_from("King").unwrap();
        let hill = Word::try_from("Hill").unwrap();

        let spelling = Spelling::try_from(vec![king, hill]).unwrap();

        let spelling_text = spelling.to_string();
        assert_eq!(spelling_text, "King Hill");
    }

    #[test]
    fn no_spelling_comes_of_no_word() {
        let words = Vec::new();

        let spelling = Spelling::try_from(words);

        assert_eq!(spelling, Err(Error::Term));
    }

    #[test]
    fn two_spellings_that_read_alike_are_the_same_spelling() {
        let own_language = Word::try_from("Treasure").unwrap();
        let lower_word = Word::try_from("treasure").unwrap();

        let first_error = Spelling::try_from(vec![own_language]).unwrap();
        let second_error = Spelling::try_from(vec![lower_word]).unwrap();

        assert_eq!(first_error, second_error);
    }
}
