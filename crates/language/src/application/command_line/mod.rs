mod outcome;
mod service;

use crate::application::command_line::outcome::Outcome;
use crate::application::command_line::service::Gate;
use crate::application::command_line::service::Press;
use crate::core::Error;
use crate::core::Languages;
use crate::core::Reader;
use crate::core::Writer;
use crate::infra;
use clap::Parser;
use ddd::{Layered, PresentationLayer};
use std::io::{self, Write, stderr, stdout};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Parser)]
#[command(name = "language", about = "The gate of the ubiquitous language.")]
pub struct CommandLine {
    #[arg(required_unless_present = "book", value_name = "SOURCE")]
    sources: Vec<PathBuf>,

    #[arg(long)]
    book: bool,
}

impl Layered for CommandLine {
    type Layer = PresentationLayer;
}

impl CommandLine {
    pub fn try_respond(&self, languages: Result<Languages, Error>) -> io::Result<ExitCode> {
        let mut report = stdout();
        let mut errors = stderr();
        let languages = match languages {
            Ok(languages) => languages,
            Err(error) => {
                writeln!(errors, "{error}")?;
                return Ok(ExitCode::FAILURE);
            }
        };
        let reader = infra::reader();
        let writer = infra::writer();
        let outcome = self.act(languages, reader, writer, &mut report, &mut errors)?;
        Ok(ExitCode::from(outcome))
    }

    fn act(
        &self,
        languages: Languages,
        reader: Box<dyn Reader>,
        writer: Box<dyn Writer>,
        report: &mut dyn Write,
        errors: &mut dyn Write,
    ) -> io::Result<Outcome> {
        match self.book {
            true => self.try_write_books(languages, writer, &self.sources, report, errors),
            false => self.try_judge_sources(languages, reader, errors),
        }
    }

    fn try_judge_sources(
        &self,
        languages: Languages,
        reader: Box<dyn Reader>,
        errors: &mut dyn Write,
    ) -> io::Result<Outcome> {
        let gate = Gate::from((languages, reader));
        let mut outcome = Outcome::Done;
        for path in &self.sources {
            if let Err(error) = gate.try_judge(path) {
                writeln!(errors, "{error}")?;
                outcome = Outcome::Error;
            }
        }
        Ok(outcome)
    }

    fn try_write_books(
        &self,
        languages: Languages,
        writer: Box<dyn Writer>,
        directories: &[PathBuf],
        report: &mut dyn Write,
        errors: &mut dyn Write,
    ) -> io::Result<Outcome> {
        let press = Press::from((languages, writer));
        let books = match press.try_gather(directories) {
            Ok(books) => books,
            Err(error) => {
                writeln!(errors, "{error}")?;
                return Ok(Outcome::Error);
            }
        };
        let mut outcome = Outcome::Done;
        for book in &books {
            if directories.is_empty() && !press.has_directory(book) {
                let book_text = book.path().display();
                writeln!(report, "[UBIQUITOUS] {book_text} is nowhere, so no book.")?;
                continue;
            }
            match press.try_write(book) {
                Ok(()) => {
                    let book_text = book.path().display();
                    let count = book.count();
                    writeln!(report, "[UBIQUITOUS] {book_text}, {count} terms.")?;
                }
                Err(error) => {
                    writeln!(errors, "{error}")?;
                    outcome = Outcome::Error;
                }
            }
        }
        Ok(outcome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Context;
    use crate::core::Language;
    use ddd::{DrivenPort, InfrastructureLayer, Port};
    use std::path::Path;

    #[test]
    fn an_error_that_reaches_no_reader_ends_the_command_line() {
        struct Shut;
        impl Write for Shut {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("shut"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::other("shut"))
            }
        }
        let words = ["language", "nowhere/one.rs"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let mut report = Vec::new();
        let mut errors = Shut;

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line.act(
            languages,
            crate::infra::reader(),
            crate::infra::writer(),
            &mut report,
            &mut errors,
        );

        assert!(outcome.is_err());
        assert!(errors.flush().is_err());
    }

    #[test]
    fn a_directory_of_no_context_that_reaches_no_reader_ends_the_command_line() {
        struct Shut;
        impl Write for Shut {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("shut"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::other("shut"))
            }
        }
        let words = ["language", "--book", "crates/nowhere"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let mut report = Vec::new();
        let mut errors = Shut;

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line.act(
            languages,
            crate::infra::reader(),
            crate::infra::writer(),
            &mut report,
            &mut errors,
        );

        assert!(outcome.is_err());
        assert!(errors.flush().is_err());
    }

    #[test]
    fn a_book_of_nowhere_that_reaches_no_reader_ends_the_command_line() {
        struct Nowhere;
        impl Layered for Nowhere {
            type Layer = InfrastructureLayer;
        }
        impl Port for Nowhere {}
        impl DrivenPort for Nowhere {}
        impl Writer for Nowhere {
            fn try_write(&self, _: &Path, _: &str) -> io::Result<()> {
                Ok(())
            }
            fn has_directory(&self, _: &Path) -> bool {
                false
            }
        }
        struct Shut;
        impl Write for Shut {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("shut"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::other("shut"))
            }
        }
        let words = ["language", "--book"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let response = Nowhere.try_write(Path::new("one"), "two");
        let mut report = Shut;
        let mut errors = Vec::new();

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line.act(
            languages,
            crate::infra::reader(),
            Box::new(Nowhere) as Box<dyn Writer>,
            &mut report,
            &mut errors,
        );

        assert_eq!(response.ok(), Some(()));
        assert!(outcome.is_err());
        assert!(report.flush().is_err());
    }

    #[test]
    fn a_book_of_terms_that_reaches_no_reader_ends_the_command_line() {
        struct Shelf;
        impl Layered for Shelf {
            type Layer = InfrastructureLayer;
        }
        impl Port for Shelf {}
        impl DrivenPort for Shelf {}
        impl Writer for Shelf {
            fn try_write(&self, _: &Path, _: &str) -> io::Result<()> {
                Ok(())
            }
            fn has_directory(&self, _: &Path) -> bool {
                true
            }
        }
        struct Shut;
        impl Write for Shut {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("shut"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::other("shut"))
            }
        }
        let words = ["language", "--book"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let mut report = Shut;
        let mut errors = Vec::new();

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line.act(
            languages,
            crate::infra::reader(),
            Box::new(Shelf) as Box<dyn Writer>,
            &mut report,
            &mut errors,
        );

        assert!(outcome.is_err());
        assert!(report.flush().is_err());
    }

    #[test]
    fn an_error_of_the_writer_that_reaches_no_reader_ends_the_command_line() {
        struct Wall;
        impl Layered for Wall {
            type Layer = InfrastructureLayer;
        }
        impl Port for Wall {}
        impl DrivenPort for Wall {}
        impl Writer for Wall {
            fn try_write(&self, _: &Path, _: &str) -> io::Result<()> {
                Err(io::Error::other("no room"))
            }
            fn has_directory(&self, _: &Path) -> bool {
                true
            }
        }
        struct Shut;
        impl Write for Shut {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("shut"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::other("shut"))
            }
        }
        let words = ["language", "--book"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let mut report = Vec::new();
        let mut errors = Shut;

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line.act(
            languages,
            crate::infra::reader(),
            Box::new(Wall) as Box<dyn Writer>,
            &mut report,
            &mut errors,
        );

        assert!(outcome.is_err());
        assert!(errors.flush().is_err());
    }

    #[test]
    fn every_book_of_every_context_reaches_a_writer_with_room() {
        struct Shelf;
        impl Layered for Shelf {
            type Layer = InfrastructureLayer;
        }
        impl Port for Shelf {}
        impl DrivenPort for Shelf {}
        impl Writer for Shelf {
            fn try_write(&self, _: &Path, _: &str) -> io::Result<()> {
                Ok(())
            }
            fn has_directory(&self, _: &Path) -> bool {
                true
            }
        }
        let words = ["language", "--book"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let reader = crate::infra::reader();
        let writer = Box::new(Shelf) as Box<dyn Writer>;
        let mut report = Vec::new();
        let mut errors = Vec::new();

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line
            .act(languages, reader, writer, &mut report, &mut errors)
            .unwrap();

        let report_text = String::from_utf8(report).unwrap();
        assert_eq!(outcome, Outcome::Done);
        assert!(report_text.contains("terms."));
    }

    #[test]
    fn a_book_whose_directory_is_nowhere_is_passed_over_and_named() {
        struct Nowhere;
        impl Layered for Nowhere {
            type Layer = InfrastructureLayer;
        }
        impl Port for Nowhere {}
        impl DrivenPort for Nowhere {}
        impl Writer for Nowhere {
            fn try_write(&self, _: &Path, _: &str) -> io::Result<()> {
                Ok(())
            }
            fn has_directory(&self, _: &Path) -> bool {
                false
            }
        }
        let words = ["language", "--book"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let book_words = ["language", "--book", "crates/harbour"];
        let book_line = CommandLine::try_parse_from(book_words).unwrap();
        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let book_languages = languages.clone();
        let reader = crate::infra::reader();
        let book_reader = crate::infra::reader();
        let writer = Box::new(Nowhere) as Box<dyn Writer>;
        let book_writer = Box::new(Nowhere) as Box<dyn Writer>;
        let mut report = Vec::new();
        let mut errors = Vec::new();

        let outcome = line
            .act(languages, reader, writer, &mut report, &mut errors)
            .unwrap();
        let book_outcome = book_line
            .act(
                book_languages,
                book_reader,
                book_writer,
                &mut report,
                &mut errors,
            )
            .unwrap();

        let report_text = String::from_utf8(report).unwrap();
        assert_eq!(outcome, Outcome::Done);
        assert_eq!(book_outcome, Outcome::Done);
        assert!(report_text.contains("is nowhere, so no book."));
        assert!(report_text.contains("terms."));
    }

    #[test]
    fn a_book_the_writer_cannot_take_leaves_the_command_line_in_error() {
        struct Wall;
        impl Layered for Wall {
            type Layer = InfrastructureLayer;
        }
        impl Port for Wall {}
        impl DrivenPort for Wall {}
        impl Writer for Wall {
            fn try_write(&self, _: &Path, _: &str) -> io::Result<()> {
                Err(io::Error::other("no room"))
            }
            fn has_directory(&self, _: &Path) -> bool {
                true
            }
        }
        let words = ["language", "--book"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let reader = crate::infra::reader();
        let writer = Box::new(Wall) as Box<dyn Writer>;
        let mut report = Vec::new();
        let mut errors = Vec::new();

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line
            .act(languages, reader, writer, &mut report, &mut errors)
            .unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        assert_eq!(outcome, Outcome::Error);
        assert!(error_text.contains("no room"));
    }

    #[test]
    fn a_command_line_without_a_source_is_no_command_line() {
        let words = ["language"];

        let line = CommandLine::try_parse_from(words);

        assert!(line.is_err());
    }

    #[test]
    fn a_command_line_of_words_yields_every_source_it_names() {
        let words = ["language", "one/mod.rs", "two/mod.rs"];

        let line = CommandLine::try_parse_from(words).unwrap();

        let sources = vec![PathBuf::from("one/mod.rs"), PathBuf::from("two/mod.rs")];
        assert_eq!(line.sources, sources);
    }

    #[test]
    fn every_source_is_read_before_the_gate_reports_its_errors() {
        let words = ["language", "nowhere/one.rs", "nowhere/two.rs"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let reader = crate::infra::reader();
        let writer = crate::infra::writer();
        let mut report = Vec::new();
        let mut errors = Vec::new();

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line
            .act(languages, reader, writer, &mut report, &mut errors)
            .unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        let count = error_text.lines().count();
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(count, 2);
    }

    #[test]
    fn a_source_in_the_language_of_its_context_leaves_the_command_line_done() {
        struct Source;
        impl Layered for Source {
            type Layer = InfrastructureLayer;
        }
        impl Port for Source {}
        impl DrivenPort for Source {}
        impl Reader for Source {
            fn try_read(&self, _: &Path) -> io::Result<String> {
                Ok(String::from("struct Treasure;"))
            }
        }
        let source = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/harbour/src/lib.rs"
        );
        let words = ["language", source];
        let line = CommandLine::try_parse_from(words).unwrap();
        let reader = Box::new(Source) as Box<dyn Reader>;
        let writer = crate::infra::writer();
        let mut report = Vec::new();
        let mut errors = Vec::new();

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line
            .act(languages, reader, writer, &mut report, &mut errors)
            .unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        assert_eq!(outcome, Outcome::Done);
        assert!(error_text.is_empty());
    }

    #[test]
    fn a_command_line_of_the_books_alone_names_no_directory() {
        let words = ["language", "--book"];

        let line = CommandLine::try_parse_from(words).unwrap();

        assert!(line.book);
        assert!(line.sources.is_empty());
    }

    #[test]
    fn a_command_line_of_the_books_yields_every_directory_it_names() {
        let words = ["language", "--book", "crates/bytes", "crates/sony"];

        let line = CommandLine::try_parse_from(words).unwrap();

        let directories = vec![PathBuf::from("crates/bytes"), PathBuf::from("crates/sony")];
        assert!(line.book);
        assert_eq!(line.sources, directories);
    }

    #[test]
    fn the_paths_are_the_same_and_the_flag_states_what_they_are_for() {
        let words = ["language", "crates/bytes"];

        let line = CommandLine::try_parse_from(words).unwrap();

        assert!(!line.book);
        assert_eq!(line.sources, vec![PathBuf::from("crates/bytes")]);
    }

    #[test]
    fn a_directory_of_no_context_is_the_book_of_nobody() {
        let words = ["language", "--book", "crates/nowhere"];
        let line = CommandLine::try_parse_from(words).unwrap();
        let reader = crate::infra::reader();
        let writer = crate::infra::writer();
        let mut report = Vec::new();
        let mut errors = Vec::new();

        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let outcome = line
            .act(languages, reader, writer, &mut report, &mut errors)
            .unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(
            error_text,
            "[LANGUAGE] crates/nowhere is covered by no context.\n"
        );
    }
}
