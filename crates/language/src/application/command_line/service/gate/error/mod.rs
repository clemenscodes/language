use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Error {
    Content {
        path: PathBuf,
        cause: String,
    },
    Language {
        path: PathBuf,
        unknown_words: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Content { path, cause } => {
                let path_text = path.display();
                write!(
                    formatter,
                    "[LANGUAGE] {path_text} could not be read, {cause}."
                )
            }
            Self::Language {
                path,
                unknown_words,
            } => {
                let path_text = path.display();
                write!(formatter, "[LANGUAGE] {path_text} writes {unknown_words}.")
            }
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_source_that_could_not_be_read_names_the_path_and_the_cause() {
        let path = PathBuf::from("crates/bytes/src/lib.rs");
        let cause = String::from("No such file or directory (os error 2)");

        let error = Error::Content { path, cause };

        let error_text = error.to_string();
        assert_eq!(
            error_text,
            "[LANGUAGE] crates/bytes/src/lib.rs could not be read, No such file or directory (os error 2)."
        );
    }

    #[test]
    fn a_source_that_writes_an_unknown_word_names_the_path_and_the_word() {
        let path = PathBuf::from("crates/bytes/src/lib.rs");
        let unknown_words = String::from("helper");

        let error = Error::Language {
            path,
            unknown_words,
        };

        let error_text = error.to_string();
        assert_eq!(
            error_text,
            "[LANGUAGE] crates/bytes/src/lib.rs writes helper."
        );
    }
}
