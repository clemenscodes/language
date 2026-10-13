use crate::core::language::vocabulary::name;
use crate::core::language::vocabulary::word;
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Word { word: word::Error },
    Name { name: name::Error },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Word { word } => write!(formatter, "{word}"),
            Self::Name { name } => write!(formatter, "{name}"),
        }
    }
}

impl error::Error for Error {}

impl From<word::Error> for Error {
    fn from(word: word::Error) -> Self {
        Self::Word { word }
    }
}

impl From<name::Error> for Error {
    fn from(name: name::Error) -> Self {
        Self::Name { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_vocabulary_states_the_failure_of_the_word_it_was_admitted_with() {
        let text = String::from("clan tag");
        let word = word::Error::Word { text };

        let error = Error::from(word);

        let error_text = error.to_string();
        assert_eq!(error_text, "[LANGUAGE] clan tag is no word.");
    }
}
