use crate::core::language::vocabulary;
use crate::core::language::vocabulary::term;
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Term { term: term::Error },
    Vocabulary { vocabulary: vocabulary::Error },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Term { term } => write!(formatter, "{term}"),
            Self::Vocabulary { vocabulary } => write!(formatter, "{vocabulary}"),
        }
    }
}

impl error::Error for Error {}

impl From<term::Error> for Error {
    fn from(term: term::Error) -> Self {
        Self::Term { term }
    }
}

impl From<vocabulary::Error> for Error {
    fn from(vocabulary: vocabulary::Error) -> Self {
        Self::Vocabulary { vocabulary }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::language::vocabulary::term::spelling;
    use crate::core::language::vocabulary::word;

    #[test]
    fn a_language_states_the_failure_of_the_term_it_was_defined_with() {
        let spelling = spelling::Error::Term;
        let term = term::Error::Spelling { spelling };

        let error = Error::from(term);

        let error_text = error.to_string();
        assert_eq!(error_text, "[LANGUAGE] a term needs at least one word.");
    }

    #[test]
    fn a_language_states_the_failure_of_the_vocabulary_it_was_admitted_with() {
        let text = String::from("clan tag");
        let word = word::Error::Word { text };
        let vocabulary = vocabulary::Error::Word { word };

        let error = Error::from(vocabulary);

        let error_text = error.to_string();
        assert_eq!(error_text, "[LANGUAGE] clan tag is no word.");
    }
}
