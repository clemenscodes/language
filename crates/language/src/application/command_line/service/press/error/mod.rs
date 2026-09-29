use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Error {
    Book { path: PathBuf, cause: String },
    Vocabulary { path: PathBuf },
    Directory { path: PathBuf },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Book { path, cause } => {
                let path_text = path.display();
                write!(
                    formatter,
                    "[LANGUAGE] {path_text} could not go out, {cause}."
                )
            }
            Self::Vocabulary { path } => {
                let path_text = path.display();
                write!(
                    formatter,
                    "[LANGUAGE] {path_text} is of no term at all, so the vocabulary was not read."
                )
            }
            Self::Directory { path } => {
                let path_text = path.display();
                write!(
                    formatter,
                    "[LANGUAGE] {path_text} is covered by no context."
                )
            }
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_book_that_could_not_go_out_names_the_path_and_the_cause() {
        let path = PathBuf::from("crates/bytes/docs/UBIQUITOUS_LANGUAGE.md");
        let cause = String::from("No such file or directory (os error 2)");

        let error = Error::Book { path, cause };

        let error_text = error.to_string();
        assert_eq!(
            error_text,
            "[LANGUAGE] crates/bytes/docs/UBIQUITOUS_LANGUAGE.md could not go out, No such file or directory (os error 2)."
        );
    }

    #[test]
    fn a_book_of_no_term_states_that_the_vocabulary_was_not_read() {
        let path = PathBuf::from("crates/bytes/docs/UBIQUITOUS_LANGUAGE.md");

        let error = Error::Vocabulary { path };

        let error_text = error.to_string();
        assert_eq!(
            error_text,
            "[LANGUAGE] crates/bytes/docs/UBIQUITOUS_LANGUAGE.md is of no term at all, so the vocabulary was not read."
        );
    }

    #[test]
    fn a_directory_of_no_context_is_named_in_the_failure() {
        let path = PathBuf::from("crates/nowhere");

        let error = Error::Directory { path };

        let error_text = error.to_string();
        assert_eq!(
            error_text,
            "[LANGUAGE] crates/nowhere is covered by no context."
        );
    }
}
