mod source;

pub(crate) mod error;
pub(crate) mod name;
pub(crate) mod term;
pub(crate) mod word;

pub(crate) use error::Error;
pub(crate) use term::Term;

pub(super) use name::Name;
pub(super) use source::Source;
pub(super) use word::Word;

use ddd::{DomainLayer, Layered, ValueObject};
use std::collections::BTreeSet;
use std::fmt;
use std::ops::{BitOr, Sub};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct Vocabulary {
    words: BTreeSet<Word>,
    names: BTreeSet<Name>,
}

impl Layered for Vocabulary {
    type Layer = DomainLayer;
}

impl ValueObject for Vocabulary {}

impl Vocabulary {
    pub(crate) fn is_empty(&self) -> bool {
        self.words.is_empty() && self.names.is_empty()
    }

    pub(super) fn define(self, term: &Term) -> Self {
        let mut words = self.words;
        for word in term {
            let taken_words = word.clone();
            words.insert(taken_words);
        }
        let names = self.names;
        Self { words, names }
    }

    pub(super) fn try_admit_names<IntoNames, IntoName>(
        self,
        parts: IntoNames,
    ) -> Result<Self, Error>
    where
        IntoNames: IntoIterator<Item = IntoName>,
        IntoName: TryInto<Name, Error = name::Error>,
    {
        let words = self.words;
        let mut names = self.names;
        for parts in parts {
            let name = parts.try_into()?;
            names.insert(name);
        }
        Ok(Self { words, names })
    }

    pub(super) fn try_admit<IntoWords, IntoWord>(self, parts: IntoWords) -> Result<Self, Error>
    where
        IntoWords: IntoIterator<Item = IntoWord>,
        IntoWord: TryInto<Word, Error = word::Error>,
    {
        let mut words = Vec::new();
        for parts in parts {
            let word = parts.try_into()?;
            words.push(word);
        }
        let vocabulary: Self = words.into_iter().collect();
        Ok(&self | &vocabulary)
    }

    pub(super) fn unknown_vocabulary(&self, source: &Source) -> Self {
        let unknown_names = source
            .names()
            .iter()
            .filter(|name| !self.names.contains(name));
        let own_language: Self = unknown_names.flatten().cloned().collect();
        let language = &own_language - self;
        let unknown = language
            .words
            .into_iter()
            .filter(|word| !self.inflects(word));
        unknown.collect()
    }

    fn inflects(&self, word: &Word) -> bool {
        let stems = word.stems();
        stems.iter().any(|stem| self.words.contains(stem))
    }
}

impl FromIterator<Word> for Vocabulary {
    fn from_iter<IntoWords: IntoIterator<Item = Word>>(parts: IntoWords) -> Self {
        let words = parts.into_iter().collect();
        let names = BTreeSet::new();
        Self { words, names }
    }
}

impl Sub for &Vocabulary {
    type Output = Vocabulary;

    fn sub(self, vocabulary: Self) -> Self::Output {
        let words = self.words.difference(&vocabulary.words).cloned().collect();
        let names = self.names.difference(&vocabulary.names).cloned().collect();
        Vocabulary { words, names }
    }
}

impl BitOr for &Vocabulary {
    type Output = Vocabulary;

    fn bitor(self, vocabulary: Self) -> Self::Output {
        let words = self.words.union(&vocabulary.words).cloned().collect();
        let names = self.names.union(&vocabulary.names).cloned().collect();
        Vocabulary { words, names }
    }
}

impl fmt::Display for Vocabulary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word_texts: Vec<String> = self.words.iter().map(Word::to_string).collect();
        let list = word_texts.join(", ");
        fmt::Display::fmt(&list, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_that_are_no_word_are_admitted_by_no_vocabulary() {
        let vocabulary = Vocabulary::default();

        let response = vocabulary.try_admit(["not a word"]);

        assert!(response.is_err());
    }

    #[test]
    fn a_vocabulary_is_the_words_it_was_gathered_from() {
        let season = Word::try_from("Season").unwrap();
        let profile = Word::try_from("Profile").unwrap();

        let vocabulary: Vocabulary = [season, profile].into_iter().collect();

        let words = BTreeSet::from([
            Word::try_from("Season").unwrap(),
            Word::try_from("Profile").unwrap(),
        ]);
        let names = BTreeSet::new();
        assert_eq!(vocabulary, Vocabulary { words, names });
    }

    #[test]
    fn the_same_word_in_two_writings_is_kept_once() {
        let lower_word = Word::try_from("seat").unwrap();
        let upper_word = Word::try_from("Seat").unwrap();

        let vocabulary: Vocabulary = [lower_word, upper_word].into_iter().collect();

        let words = BTreeSet::from([Word::try_from("seat").unwrap()]);
        let names = BTreeSet::new();
        assert_eq!(vocabulary, Vocabulary { words, names });
    }

    #[test]
    fn a_term_puts_every_one_of_its_words_into_the_vocabulary() {
        let vocabulary = Vocabulary::default();
        let term =
            Term::try_from((["King", "of", "the", "Hill"], "The matchtype with a hill.")).unwrap();

        let language = vocabulary.define(&term);

        let king = Word::try_from("King").unwrap();
        let hill = Word::try_from("Hill").unwrap();
        let own_language: Vocabulary = [king, hill].into_iter().collect();
        let words = &own_language - &language;
        assert!(words.is_empty());
    }

    #[test]
    fn a_word_the_other_vocabulary_lacks_is_left_over() {
        let anchor = Word::try_from("anchor").unwrap();
        let treasure = Word::try_from("treasure").unwrap();
        let own_language: Vocabulary = [anchor].into_iter().collect();
        let language: Vocabulary = [treasure].into_iter().collect();

        let words = &own_language - &language;

        assert_eq!(words, own_language);
    }

    #[test]
    fn a_word_the_other_vocabulary_has_is_taken_away() {
        let treasure = Word::try_from("treasure").unwrap();
        let own_language: Vocabulary = [treasure.clone()].into_iter().collect();
        let language: Vocabulary = [treasure].into_iter().collect();

        let words = &own_language - &language;

        assert!(words.is_empty());
    }

    #[test]
    fn a_word_is_taken_away_whatever_its_writing() {
        let upper_word = Word::try_from("Treasure").unwrap();
        let lower_word = Word::try_from("treasure").unwrap();
        let own_language: Vocabulary = [upper_word].into_iter().collect();
        let language: Vocabulary = [lower_word].into_iter().collect();

        let words = &own_language - &language;

        assert!(words.is_empty());
    }

    #[test]
    fn only_the_words_that_are_left_over_remain() {
        let seat = Word::try_from("Seat").unwrap();
        let anchor = Word::try_from("Anchor").unwrap();
        let own_language: Vocabulary = [seat.clone(), anchor].into_iter().collect();
        let language: Vocabulary = [seat].into_iter().collect();

        let words = &own_language - &language;

        let anchor_words = BTreeSet::from([Word::try_from("Anchor").unwrap()]);
        let names = BTreeSet::new();
        assert_eq!(
            words,
            Vocabulary {
                words: anchor_words,
                names
            }
        );
    }

    #[test]
    fn a_word_in_the_plural_is_no_other_word_than_the_one_it_stems_from() {
        let vocabulary = Vocabulary::default();
        let term =
            Term::try_from((["Entry"], "One name together with the number it yields.")).unwrap();
        let language = vocabulary.define(&term);

        let unknown_vocabulary = language.unknown_vocabulary(&Source::from("entries"));

        assert!(unknown_vocabulary.is_empty());
    }

    #[test]
    fn a_plural_that_ends_in_ses_is_no_other_word_than_the_one_that_ends_in_se() {
        let vocabulary = Vocabulary::default();
        let term = Term::try_from((["Phrase"], "The words a text shows.")).unwrap();
        let language = vocabulary.define(&term);

        let unknown_vocabulary = language.unknown_vocabulary(&Source::from("phrases"));

        assert!(unknown_vocabulary.is_empty());
    }

    #[test]
    fn a_word_whose_stem_belongs_to_nobody_is_an_error_all_the_same() {
        let vocabulary = Vocabulary::default();
        let term =
            Term::try_from((["Entry"], "One name together with the number it yields.")).unwrap();
        let language = vocabulary.define(&term);

        let unknown_vocabulary = language.unknown_vocabulary(&Source::from("treasures"));

        assert!(!unknown_vocabulary.is_empty());
    }

    #[test]
    fn an_identifier_admitted_as_a_name_is_no_error() {
        let vocabulary = Vocabulary::default();

        let language = vocabulary.try_admit_names(["is_empty"]).unwrap();

        let unknown_vocabulary = language.unknown_vocabulary(&Source::from("text.is_empty()"));
        let unknown_text = unknown_vocabulary.to_string();
        assert_eq!(unknown_text, "text");
    }

    #[test]
    fn a_word_of_an_identifier_admitted_as_a_name_is_no_word_of_its_own() {
        let vocabulary = Vocabulary::default();

        let language = vocabulary.try_admit_names(["is_empty"]).unwrap();

        let unknown_vocabulary = language.unknown_vocabulary(&Source::from("fn is_port() {}"));
        let unknown_text = unknown_vocabulary.to_string();
        assert_eq!(unknown_text, "fn, is, port");
    }

    #[test]
    fn letters_that_are_no_name_are_admitted_by_no_vocabulary() {
        let vocabulary = Vocabulary::default();

        let response = vocabulary.try_admit_names(["is empty"]);

        assert!(response.is_err());
    }

    #[test]
    fn a_vocabulary_is_shown_as_its_words_apart_by_a_comma() {
        let words = BTreeSet::from([
            Word::try_from("seat").unwrap(),
            Word::try_from("chest").unwrap(),
        ]);
        let names = BTreeSet::new();
        let vocabulary = Vocabulary { words, names };

        let vocabulary_text = vocabulary.to_string();

        assert_eq!(vocabulary_text, "chest, seat");
    }

    #[test]
    fn an_empty_vocabulary_is_shown_as_nothing() {
        let vocabulary = Vocabulary::default();

        let vocabulary_text = vocabulary.to_string();

        assert_eq!(vocabulary_text, "");
    }
}
