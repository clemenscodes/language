use crate::core::language::vocabulary::term::definition;
use crate::core::language::vocabulary::term::spelling;
use crate::core::language::vocabulary::word;
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Word { word: word::Error },
    Spelling { spelling: spelling::Error },
    Definition { definition: definition::Error },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Word { word } => write!(formatter, "{word}"),
            Self::Spelling { spelling } => write!(formatter, "{spelling}"),
            Self::Definition { definition } => write!(formatter, "{definition}"),
        }
    }
}

impl error::Error for Error {}

impl From<word::Error> for Error {
    fn from(word: word::Error) -> Self {
        Self::Word { word }
    }
}

impl From<spelling::Error> for Error {
    fn from(spelling: spelling::Error) -> Self {
        Self::Spelling { spelling }
    }
}

impl From<definition::Error> for Error {
    fn from(definition: definition::Error) -> Self {
        Self::Definition { definition }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_term_states_the_failure_of_the_word_it_was_spelled_with() {
        let text = String::from("clan tag");
        let word = word::Error::Word { text };

        let error = Error::from(word);

        let error_text = error.to_string();
        assert_eq!(error_text, "[LANGUAGE] clan tag is no word.");
    }

    #[test]
    fn a_term_states_the_failure_of_the_spelling_it_was_named_by() {
        let spelling = spelling::Error::Term;

        let error = Error::from(spelling);

        let error_text = error.to_string();
        assert_eq!(error_text, "[LANGUAGE] a term needs at least one word.");
    }

    #[test]
    fn a_term_states_the_failure_of_the_definition_it_takes() {
        let text = String::from("   ");
        let definition = definition::Error::Definition { text };

        let error = Error::from(definition);

        let error_text = error.to_string();
        assert_eq!(error_text, "[LANGUAGE]     is no definition.");
    }
}
