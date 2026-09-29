pub(crate) mod error;

pub(crate) use error::Error;

use ddd::{DomainLayer, Layered, ValueObject};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq)]
pub struct Word {
    text: String,
}

impl Layered for Word {
    type Layer = DomainLayer;
}

impl ValueObject for Word {}

impl Word {
    const DOUBLE: &'static str = "ss";
    const PLURAL: &'static str = "ie";
    const SIBILANT: &'static str = "sxzh";
    const MANY: &'static str = "s";
    const SIBILANT_MANY: &'static str = "es";

    pub(super) fn stems(&self) -> Vec<Self> {
        let text = self.lower_case();
        if text.ends_with(Self::DOUBLE) {
            return Vec::new();
        }
        let Some(singular) = text.strip_suffix(Self::MANY) else {
            return Vec::new();
        };
        let plural = singular.strip_suffix(Self::PLURAL);
        if let Some(kept_word) = plural {
            let stem = format!("{kept_word}y");
            return Self::try_from(stem.as_str()).into_iter().collect();
        }
        let mut stems = Vec::new();
        let sibilant_plural = text.strip_suffix(Self::SIBILANT_MANY);
        if let Some(kept_word) = sibilant_plural {
            let last_letter = kept_word.chars().last();
            let sibilant = last_letter.is_some_and(|letter| Self::SIBILANT.contains(letter));
            if sibilant {
                stems.extend(Self::try_from(kept_word));
            }
        }
        stems.extend(Self::try_from(singular));
        stems
    }

    fn lower_case(&self) -> String {
        self.text.to_lowercase()
    }
}

impl TryFrom<&str> for Word {
    type Error = Error;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let letters = text.chars().all(|letter| letter.is_ascii_alphanumeric());
        let alphabetic_start = text
            .chars()
            .next()
            .is_some_and(|letter| letter.is_ascii_alphabetic());
        let text = text.to_string();
        if text.is_empty() || !letters || !alphabetic_start {
            return Err(Error::Word { text });
        }
        Ok(Self { text })
    }
}

impl PartialEq for Word {
    fn eq(&self, word: &Self) -> bool {
        self.lower_case() == word.lower_case()
    }
}

impl Hash for Word {
    fn hash<IntoHasher: Hasher>(&self, hasher: &mut IntoHasher) {
        self.lower_case().hash(hasher);
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, word: &Self) -> Option<Ordering> {
        Some(self.cmp(word))
    }
}

impl Ord for Word {
    fn cmp(&self, word: &Self) -> Ordering {
        self.lower_case().cmp(&word.lower_case())
    }
}

impl fmt::Display for Word {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.text, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_word_in_two_writings_reaches_one_entry_of_a_table() {
        use std::collections::HashSet;
        let lower_word = Word::try_from("seat").unwrap();
        let upper_word = Word::try_from("Seat").unwrap();

        let table: HashSet<Word> = [lower_word, upper_word].into_iter().collect();

        assert_eq!(table.len(), 1);
    }

    #[test]
    fn two_writings_of_the_same_word_are_the_same_word() {
        let upper_word = Word::try_from("TREASURE").unwrap();
        let lower_word = Word::try_from("treasure").unwrap();

        let sameness = upper_word == lower_word;

        assert!(sameness);
    }

    #[test]
    fn two_different_words_are_not_the_same_word() {
        let chest = Word::try_from("chest").unwrap();
        let treasure = Word::try_from("treasure").unwrap();

        let sameness = chest == treasure;

        assert!(!sameness);
    }

    #[test]
    fn two_writings_of_the_same_word_are_ordered_together() {
        let upper_word = Word::try_from("TREASURE").unwrap();
        let lower_word = Word::try_from("treasure").unwrap();

        let order = upper_word.cmp(&lower_word);

        assert_eq!(order, Ordering::Equal);
    }

    #[test]
    fn words_are_ordered_by_their_letters_and_not_by_their_case() {
        let chest = Word::try_from("Chest").unwrap();
        let treasure = Word::try_from("treasure").unwrap();

        let order = chest.cmp(&treasure);

        assert_eq!(order, Ordering::Less);
    }

    #[test]
    fn a_word_is_shown_in_the_case_it_came_in() {
        let word = Word::try_from("Plunder").unwrap();

        let word_text = word.to_string();

        assert_eq!(word_text, "Plunder");
    }

    #[test]
    fn a_word_may_end_in_digits() {
        let text = "ak47";

        let word = Word::try_from(text);

        assert!(word.is_ok());
    }

    #[test]
    fn digits_alone_are_not_a_word() {
        let text = "40135";

        let word = Word::try_from(text);

        let text = String::from("40135");
        assert_eq!(word, Err(Error::Word { text }));
    }

    #[test]
    fn nothing_is_not_a_word() {
        let text = "";

        let word = Word::try_from(text);

        let text = String::new();
        assert_eq!(word, Err(Error::Word { text }));
    }

    #[test]
    fn two_words_are_not_one_word() {
        let text = "clan tag";

        let word = Word::try_from(text);

        let text = String::from("clan tag");
        assert_eq!(word, Err(Error::Word { text }));
    }

    #[test]
    fn a_writing_with_an_underscore_is_not_one_word() {
        let text = "season_profile";

        let word = Word::try_from(text);

        let text = String::from("season_profile");
        assert_eq!(word, Err(Error::Word { text }));
    }

    #[test]
    fn a_word_in_the_plural_stems_from_the_word_without_the_s() {
        let word = Word::try_from("letters").unwrap();

        let stems = word.stems();

        assert_eq!(stems, vec![Word::try_from("letter").unwrap()]);
    }

    #[test]
    fn a_word_that_ends_in_ies_stems_from_the_word_that_ends_in_y() {
        let word = Word::try_from("entries").unwrap();

        let stems = word.stems();

        assert_eq!(stems, vec![Word::try_from("entry").unwrap()]);
    }

    #[test]
    fn a_word_that_ends_in_es_after_a_sibilant_keeps_its_e() {
        let word = Word::try_from("matches").unwrap();

        let stems = word.stems();

        assert_eq!(
            stems,
            vec![
                Word::try_from("match").unwrap(),
                Word::try_from("matche").unwrap()
            ]
        );
    }

    #[test]
    fn a_word_that_ends_in_es_after_a_sibilant_stems_from_the_word_without_the_s_as_well() {
        let word = Word::try_from("phrases").unwrap();

        let stems = word.stems();

        assert_eq!(
            stems,
            vec![
                Word::try_from("phras").unwrap(),
                Word::try_from("phrase").unwrap()
            ]
        );
    }

    #[test]
    fn a_word_in_upper_case_stems_from_the_same_word() {
        let word = Word::try_from("NAMES").unwrap();

        let stems = word.stems();

        assert_eq!(stems, vec![Word::try_from("Name").unwrap()]);
    }

    #[test]
    fn a_word_that_ends_in_a_double_s_stems_from_no_word() {
        let word = Word::try_from("success").unwrap();

        let stems = word.stems();

        assert!(stems.is_empty());
    }

    #[test]
    fn a_word_that_ends_in_no_s_stems_from_no_word() {
        let word = Word::try_from("letter").unwrap();

        let stems = word.stems();

        assert!(stems.is_empty());
    }

    #[test]
    fn the_letter_s_alone_stems_from_no_word() {
        let word = Word::try_from("s").unwrap();

        let stems = word.stems();

        assert!(stems.is_empty());
    }
}
