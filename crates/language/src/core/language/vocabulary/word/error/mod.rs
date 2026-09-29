use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Word { text: String },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Word { text } => {
                write!(formatter, "[LANGUAGE] {text} is no word.")
            }
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_writing_that_yields_no_word_is_named_in_the_failure() {
        let text = String::from("clan tag");

        let error = Error::Word { text };

        let error_text = error.to_string();
        assert_eq!(error_text, "[LANGUAGE] clan tag is no word.");
    }

    #[test]
    fn two_failures_of_the_same_writing_are_the_same_failure() {
        let text = String::from("helper");

        let first_error = Error::Word { text: text.clone() };
        let second_error = Error::Word { text };

        assert_eq!(first_error, second_error);
    }
}
