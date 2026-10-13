pub(crate) mod error;

pub(crate) use error::Error;

use crate::core::language::vocabulary::word::Word;
use ddd::{DomainLayer, Layered, ValueObject};
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name {
    words: Vec<Word>,
}

impl Layered for Name {
    type Layer = DomainLayer;
}

impl ValueObject for Name {}

impl TryFrom<&str> for Name {
    type Error = Error;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let letters: Vec<char> = text.chars().collect();
        let identifier = letters
            .iter()
            .all(|letter| letter.is_ascii_alphanumeric() || *letter == '_');
        let mut pieces = Vec::new();
        let mut current = String::new();
        let mut position = 0;
        while position < letters.len() {
            let letter = letters[position];
            let next_letter = letters.get(position + 1).copied();
            position += 1;
            if !letter.is_ascii_alphanumeric() {
                let piece = std::mem::take(&mut current);
                if !piece.is_empty() {
                    pieces.push(piece);
                }
                continue;
            }
            if letter.is_ascii_uppercase() {
                let last_letter = current.chars().last();
                let after_lower = last_letter.is_some_and(|letter| letter.is_ascii_lowercase());
                let after_digit = last_letter.is_some_and(|letter| letter.is_ascii_digit());
                let before_lower = next_letter.is_some_and(|next| next.is_ascii_lowercase());
                let word_start = before_lower && !current.is_empty();
                if after_lower || after_digit || word_start {
                    let piece = std::mem::take(&mut current);
                    pieces.push(piece);
                }
            }
            current.push(letter);
        }
        if !current.is_empty() {
            pieces.push(current);
        }
        let words: Vec<Word> = pieces
            .iter()
            .filter_map(|piece| Word::try_from(piece.as_str()).ok())
            .collect();
        if !identifier || words.is_empty() {
            let text = text.to_string();
            return Err(Error::Name { text });
        }
        Ok(Self { words })
    }
}

impl<'a> IntoIterator for &'a Name {
    type Item = &'a Word;
    type IntoIter = Iter<'a, Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_falls_apart_into_its_words_at_the_underscore() {
        let text = "is_empty";

        let name = Name::try_from(text).unwrap();

        let words: Vec<String> = name.into_iter().map(Word::to_string).collect();
        assert_eq!(words, vec!["is", "empty"]);
    }

    #[test]
    fn a_name_falls_apart_into_its_words_at_an_upper_case_letter() {
        let text = "IntoIterator";

        let name = Name::try_from(text).unwrap();

        let words: Vec<String> = name.into_iter().map(Word::to_string).collect();
        assert_eq!(words, vec!["Into", "Iterator"]);
    }

    #[test]
    fn two_writings_of_the_same_words_are_the_same_name() {
        let snake_name = Name::try_from("as_ref").unwrap();
        let camel_name = Name::try_from("AsRef").unwrap();

        let sameness = snake_name == camel_name;

        assert!(sameness);
    }

    #[test]
    fn the_same_words_in_another_order_are_another_name() {
        let first_name = Name::try_from("is_empty").unwrap();
        let second_name = Name::try_from("empty_is").unwrap();

        let sameness = first_name == second_name;

        assert!(!sameness);
    }

    #[test]
    fn nothing_is_not_a_name() {
        let text = "";

        let name = Name::try_from(text);

        let text = String::new();
        assert_eq!(name, Err(Error::Name { text }));
    }

    #[test]
    fn an_underscore_alone_is_not_a_name() {
        let text = "_";

        let name = Name::try_from(text);

        let text = String::from("_");
        assert_eq!(name, Err(Error::Name { text }));
    }

    #[test]
    fn two_names_apart_by_a_space_are_not_one_name() {
        let text = "is empty";

        let name = Name::try_from(text);

        let text = String::from("is empty");
        assert_eq!(name, Err(Error::Name { text }));
    }
}
