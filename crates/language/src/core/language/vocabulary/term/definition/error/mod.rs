use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Definition { text: String },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Definition { text } => {
                write!(formatter, "[LANGUAGE] {text} is no definition.")
            }
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_writing_that_yields_no_definition_is_named_in_the_failure() {
        let text = String::from("   ");

        let error = Error::Definition { text };

        let error_text = error.to_string();
        assert_eq!(error_text, "[LANGUAGE]     is no definition.");
    }
}
