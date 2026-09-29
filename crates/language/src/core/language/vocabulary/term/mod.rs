pub(crate) mod definition;
pub(crate) mod error;
pub(crate) mod spelling;

pub(crate) use definition::Definition;
pub(crate) use error::Error;
pub(crate) use spelling::Spelling;

use crate::core::language::vocabulary::word;
use crate::core::language::vocabulary::word::Word;
use ddd::{DomainLayer, Entity, Layered};
use std::fmt;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Term {
    spelling: Spelling,
    definition: Definition,
}

impl Layered for Term {
    type Layer = DomainLayer;
}

impl Entity for Term {
    type Identity = Spelling;

    fn identity(&self) -> &Self::Identity {
        &self.spelling
    }
}

impl Term {
    pub(crate) fn definition(&self) -> &Definition {
        &self.definition
    }
}

impl<IntoSpelling, IntoWord, IntoDefinition> TryFrom<(IntoSpelling, IntoDefinition)> for Term
where
    IntoSpelling: IntoIterator<Item = IntoWord>,
    IntoWord: TryInto<Word, Error = word::Error>,
    IntoDefinition: TryInto<Definition, Error = definition::Error>,
{
    type Error = Error;

    fn try_from(
        (spelling, definition): (IntoSpelling, IntoDefinition),
    ) -> Result<Self, Self::Error> {
        let mut words = Vec::new();
        for each in spelling {
            let word = each.try_into()?;
            words.push(word);
        }
        let spelling = Spelling::try_from(words)?;
        let definition = definition.try_into()?;
        Ok(Self {
            spelling,
            definition,
        })
    }
}

impl<'term> IntoIterator for &'term Term {
    type Item = &'term Word;
    type IntoIter = Iter<'term, Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.spelling.into_iter()
    }
}

impl fmt::Display for Term {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.spelling, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_spelling_of_words_and_a_definition_yield_a_term() {
        let spelling = vec!["Column", "Length"];

        let term = Term::try_from((spelling, "how wide a column is")).unwrap();

        assert_eq!(term.identity().to_string(), "Column Length");
    }

    #[test]
    fn a_spelling_that_is_no_word_is_no_term() {
        let spelling = vec!["not a word"];

        let response = Term::try_from((spelling, "a definition"));

        assert!(response.is_err());
    }

    #[test]
    fn a_spelling_of_no_word_at_all_is_no_term() {
        let spelling: Vec<&str> = Vec::new();

        let response = Term::try_from((spelling, "a definition"));

        assert!(response.is_err());
    }

    #[test]
    fn a_term_without_a_definition_is_no_term() {
        let spelling = vec!["Word"];

        let response = Term::try_from((spelling, ""));

        assert!(response.is_err());
    }

    #[test]
    fn a_term_of_one_word_yields_that_word() {
        let spelling = ["Treasure"];

        let term = Term::try_from((spelling, "The object a team brings to the chest.")).unwrap();

        let words: Vec<Word> = term.into_iter().cloned().collect();
        let expected_text = vec![Word::try_from("Treasure").unwrap()];
        assert_eq!(words, expected_text);
    }

    #[test]
    fn a_term_of_several_words_yields_every_one_of_them_in_order() {
        let spelling = ["King", "of", "the", "Hill"];

        let term = Term::try_from((spelling, "The matchtype with a hill.")).unwrap();

        let words: Vec<Word> = term.into_iter().cloned().collect();
        let expected_text = vec![
            Word::try_from("King").unwrap(),
            Word::try_from("of").unwrap(),
            Word::try_from("the").unwrap(),
            Word::try_from("Hill").unwrap(),
        ];
        assert_eq!(words, expected_text);
    }

    #[test]
    fn two_terms_of_the_same_spelling_and_definition_are_the_same_term() {
        let spelling = ["Treasure"];

        let first_error =
            Term::try_from((spelling, "The object a team brings to the chest.")).unwrap();
        let second_error =
            Term::try_from((spelling, "The object a team brings to the chest.")).unwrap();

        assert_eq!(first_error, second_error);
    }

    #[test]
    fn a_term_keeps_the_definition_it_was_given() {
        let spelling = ["Treasure"];

        let term = Term::try_from((spelling, "The object a team brings to the chest.")).unwrap();
        let same_term = Term::try_from((spelling, "The object a team delivers.")).unwrap();

        assert_ne!(term, same_term);
    }

    #[test]
    fn a_term_yields_what_it_means() {
        let spelling = ["Treasure"];

        let term = Term::try_from((spelling, "The object a team brings to the chest.")).unwrap();

        let term_text = term.definition().to_string();
        assert_eq!(term_text, "The object a team brings to the chest.");
    }

    #[test]
    fn a_term_redefined_is_still_the_same_term() {
        let spelling = ["Treasure"];

        let term = Term::try_from((spelling, "The object a team brings to the chest.")).unwrap();
        let same_term = Term::try_from((spelling, "The object a team delivers.")).unwrap();

        let sameness = term.is_same_entity(&same_term);
        assert!(sameness);
        assert_ne!(term, same_term);
    }

    #[test]
    fn a_term_is_identified_by_its_spelling() {
        let spelling = ["King", "of", "the", "Hill"];
        let term = Term::try_from((spelling, "The matchtype with a hill.")).unwrap();

        let identity = term.identity();

        let identity_text = identity.to_string();
        assert_eq!(identity_text, "King of the Hill");
    }

    #[test]
    fn a_term_is_shown_as_its_words_apart_by_a_space() {
        let spelling = ["King", "of", "the", "Hill"];

        let term = Term::try_from((spelling, "The matchtype with a hill.")).unwrap();

        let term_text = term.to_string();
        assert_eq!(term_text, "King of the Hill");
    }

    #[test]
    fn a_spelling_without_a_word_yields_no_term() {
        let spelling: [&str; 0] = [];

        let term = Term::try_from((spelling, "The object a team brings to the chest."));

        let spelling = spelling::Error::Term;
        assert_eq!(term, Err(Error::Spelling { spelling }));
    }

    #[test]
    fn a_spelling_that_is_no_word_yields_no_term() {
        let spelling = ["season_profile"];

        let term = Term::try_from((spelling, "The record of one season."));

        let text = String::from("season_profile");
        let word = word::Error::Word { text };
        assert_eq!(term, Err(Error::Word { word }));
    }

    #[test]
    fn a_spelling_without_a_definition_yields_no_term() {
        let spelling = ["Treasure"];

        let term = Term::try_from((spelling, ""));

        let text = String::new();
        let definition = definition::Error::Definition { text };
        assert_eq!(term, Err(Error::Definition { definition }));
    }
}
