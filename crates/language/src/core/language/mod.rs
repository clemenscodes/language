pub(crate) mod error;
pub(crate) mod vocabulary;

pub use error::Error;

use crate::core::language::vocabulary::Source;
use crate::core::language::vocabulary::Term;
use crate::core::language::vocabulary::Vocabulary;
use crate::core::language::vocabulary::Word;
use crate::core::language::vocabulary::term;
use crate::core::language::vocabulary::word;
use ddd::{DomainLayer, Entity, Layered, Specification, ValueObject};
use std::collections::BTreeMap;
use std::ops::BitOr;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Language {
    vocabulary: Vocabulary,
    terms: BTreeMap<String, Term>,
}

impl Layered for Language {
    type Layer = DomainLayer;
}

impl ValueObject for Language {}

impl Specification<Source> for Language {
    fn is_satisfied_by(&self, candidate: &Source) -> bool {
        let unknown_vocabulary = self.vocabulary.unknown_vocabulary(candidate);
        unknown_vocabulary.is_empty()
    }
}

impl Language {
    pub fn try_define<IntoSpelling, IntoDefinition>(
        self,
        spelling: IntoSpelling,
        definition: IntoDefinition,
    ) -> Result<Self, Error>
    where
        (IntoSpelling, IntoDefinition): TryInto<Term, Error = term::Error>,
    {
        let parts = (spelling, definition);
        let term: Term = parts.try_into()?;
        let vocabulary = self.vocabulary.define(&term);
        let mut terms = self.terms;
        let spelling = term.identity().to_string();
        terms.insert(spelling, term);
        Ok(Self { vocabulary, terms })
    }

    pub fn try_admit<IntoWords, IntoWord>(self, words: IntoWords) -> Result<Self, Error>
    where
        IntoWords: IntoIterator<Item = IntoWord>,
        IntoWord: TryInto<Word, Error = word::Error>,
    {
        let vocabulary = self.vocabulary.try_admit(words)?;
        let terms = self.terms;
        Ok(Self { vocabulary, terms })
    }

    pub(crate) fn terms(&self) -> impl Iterator<Item = (String, String)> + '_ {
        self.terms
            .values()
            .map(|term| (term.to_string(), term.definition().to_string()))
    }

    pub(crate) fn unknown_vocabulary(&self, text: impl Into<Source>) -> Vocabulary {
        let source = text.into();
        self.vocabulary.unknown_vocabulary(&source)
    }
}

impl BitOr for &Language {
    type Output = Language;

    fn bitor(self, language: Self) -> Self::Output {
        let vocabulary = &self.vocabulary | &language.vocabulary;
        let mut terms = self.terms.clone();
        terms.extend(language.terms.clone());
        Language { vocabulary, terms }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_spelling_that_is_no_word_defines_no_term() {
        let language = Language::default();

        let response = language.try_define(["not a word"], "a definition");

        assert!(response.is_err());
    }

    #[test]
    fn letters_that_are_no_word_are_admitted_by_nobody() {
        let language = Language::default();

        let response = language.try_admit(["not a word"]);

        assert!(response.is_err());
    }

    #[test]
    fn a_word_of_a_term_is_no_error() {
        let language = Language::default();

        let language = language
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap();

        let unknown_vocabulary = language.unknown_vocabulary("treasure");
        assert!(unknown_vocabulary.is_empty());
    }

    #[test]
    fn a_word_of_no_term_is_an_error() {
        let language = Language::default();

        let language = language
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap();

        let unknown_vocabulary = language.unknown_vocabulary("helper");
        let unknown_text = unknown_vocabulary.to_string();
        assert_eq!(unknown_text, "helper");
    }

    #[test]
    fn every_word_of_a_term_of_several_words_belongs_to_the_language() {
        let language = Language::default();

        let language = language
            .try_define(["King", "of", "the", "Hill"], "The matchtype with a hill.")
            .unwrap();

        let unknown_vocabulary = language.unknown_vocabulary("King Hill");
        assert!(unknown_vocabulary.is_empty());
    }

    #[test]
    fn a_term_that_cannot_be_read_is_an_error() {
        let language = Language::default();

        let language = language.try_define(["season_profile"], "The record of one season.");

        let text = String::from("season_profile");
        let word = word::Error::Word { text };
        let term = term::Error::Word { word };
        assert_eq!(language, Err(Error::Term { term }));
    }

    #[test]
    fn a_text_of_nothing_but_taught_words_satisfies_the_language() {
        let language = Language::default();

        let language = language
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap();

        let source = Source::from("Treasure");
        let fit = language.is_satisfied_by(&source);
        assert!(fit);
    }

    #[test]
    fn a_text_with_a_word_of_no_term_does_not_satisfy_the_language() {
        let language = Language::default();

        let language = language
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap();

        let source = Source::from("struct Treasure {}");
        let fit = language.is_satisfied_by(&source);
        assert!(!fit);
    }

    #[test]
    fn a_language_yields_the_terms_it_was_given() {
        let language = Language::default();

        let language = language
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap();

        let spellings: Vec<String> = language.terms().map(|(spelling, _)| spelling).collect();
        assert_eq!(spellings, vec![String::from("Treasure")]);
    }

    #[test]
    fn a_word_admitted_without_a_definition_is_no_term() {
        let language = Language::default();

        let language = language.try_admit(["helper"]).unwrap();

        let count = language.terms().count();
        assert_eq!(count, 0);
    }

    #[test]
    fn one_spelling_is_one_term_however_often_it_is_defined() {
        let language = Language::default();

        let language = language
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap()
            .try_define(["Treasure"], "The object a team delivers to the chest.")
            .unwrap();

        let definitions: Vec<String> = language.terms().map(|(_, definition)| definition).collect();
        assert_eq!(
            definitions,
            vec![String::from("The object a team delivers to the chest.")]
        );
    }

    #[test]
    fn two_joined_languages_yield_the_terms_of_both() {
        let treasure_language = Language::default()
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap();
        let chest_language = Language::default()
            .try_define(["Chest"], "The chest a team delivers to.")
            .unwrap();

        let language = &treasure_language | &chest_language;

        let spellings: Vec<String> = language.terms().map(|(spelling, _)| spelling).collect();
        assert_eq!(
            spellings,
            vec![String::from("Chest"), String::from("Treasure")]
        );
    }

    #[test]
    fn a_language_without_a_term_makes_every_word_an_error() {
        let language = Language::default();

        let unknown_vocabulary = language.unknown_vocabulary("treasure");

        let unknown_text = unknown_vocabulary.to_string();
        assert_eq!(unknown_text, "treasure");
    }
}
