use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Term,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Term => {
                write!(formatter, "[LANGUAGE] a term needs at least one word.")
            }
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_term_without_a_word_states_what_a_term_needs() {
        let error = Error::Term;

        let error_text = error.to_string();

        assert_eq!(error_text, "[LANGUAGE] a term needs at least one word.");
    }
}
