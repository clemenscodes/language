pub(crate) mod error;

pub(crate) use error::Error;

use crate::core::Book;
use crate::core::Languages;
use crate::core::Writer;
use ddd::{ApplicationLayer, ApplicationService, Layered};
use std::path::PathBuf;

pub(crate) struct Press {
    languages: Languages,
    writer: Box<dyn Writer>,
}

impl Layered for Press {
    type Layer = ApplicationLayer;
}

impl ApplicationService for Press {}

impl Press {
    pub(crate) fn try_gather(&self, directories: &[PathBuf]) -> Result<Vec<Book>, Error> {
        let contexts = self.languages.contexts();
        if directories.is_empty() {
            let books = contexts.iter().map(Book::from).collect();
            return Ok(books);
        }
        let mut books = Vec::with_capacity(directories.len());
        for directory in directories {
            let context = contexts.iter().find(|context| context.covers(directory));
            let Some(context) = context else {
                let path = directory.clone();
                return Err(Error::Directory { path });
            };
            books.push(Book::from((directory.as_path(), context)));
        }
        Ok(books)
    }

    pub(crate) fn has_directory(&self, book: &Book) -> bool {
        self.writer.has_directory(book.path())
    }

    pub(crate) fn try_write(&self, book: &Book) -> Result<(), Error> {
        let count = book.count();
        if count == 0 {
            let path = book.path().to_path_buf();
            return Err(Error::Vocabulary { path });
        }
        let text = book.to_string();
        match self.writer.try_write(book.path(), &text) {
            Ok(()) => Ok(()),
            Err(cause) => {
                let path = book.path().to_path_buf();
                let cause = cause.to_string();
                Err(Error::Book { path, cause })
            }
        }
    }
}

impl From<(Languages, Box<dyn Writer>)> for Press {
    fn from((languages, writer): (Languages, Box<dyn Writer>)) -> Self {
        Self { languages, writer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Context;
    use crate::core::Language;
    use ddd::{DrivenPort, InfrastructureLayer, Port};
    use std::io;
    use std::path::Path;

    #[test]
    fn a_writer_with_room_has_the_directory_and_takes_the_book() {
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
        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let press = Press::from((languages, Box::new(Shelf) as Box<dyn Writer>));
        let books = press.try_gather(&[]).unwrap();

        let room = press.has_directory(&books[0]);
        let response = press.try_write(&books[0]);

        assert!(room);
        assert_eq!(response, Ok(()));
    }

    #[test]
    fn a_writer_without_room_has_no_directory_and_takes_no_book() {
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
                false
            }
        }
        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let press = Press::from((languages, Box::new(Wall) as Box<dyn Writer>));
        let books = press.try_gather(&[]).unwrap();

        let room = press.has_directory(&books[0]);
        let response = press.try_write(&books[0]);

        let path = books[0].path().to_path_buf();
        let cause = String::from("no room");
        assert!(!room);
        assert_eq!(response, Err(Error::Book { path, cause }));
    }

    #[test]
    fn a_press_of_no_context_at_all_yields_no_book() {
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let press = Press::from((languages, crate::infra::writer()));

        let books = press.try_gather(&[]).unwrap();

        assert!(books.is_empty());
    }

    #[test]
    fn a_named_directory_yields_the_book_of_the_context_that_covers_it() {
        let contexts = vec![Context::from(("harbour", Language::default()))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let press = Press::from((languages, crate::infra::writer()));
        let directories = vec![PathBuf::from("crates/harbour")];

        let books = press.try_gather(&directories).unwrap();

        let paths: Vec<&Path> = books.iter().map(Book::path).collect();
        assert_eq!(
            paths,
            vec![Path::new("crates/harbour/docs/UBIQUITOUS_LANGUAGE.md")]
        );
    }

    #[test]
    fn two_named_directories_yield_two_books() {
        let contexts = vec![
            Context::from(("harbour", Language::default())),
            Context::from(("ship", Language::default())),
        ];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let press = Press::from((languages, crate::infra::writer()));
        let directories = vec![
            PathBuf::from("crates/harbour"),
            PathBuf::from("crates/ship"),
        ];

        let books = press.try_gather(&directories).unwrap();

        let count = books.len();
        assert_eq!(count, 2);
    }

    #[test]
    fn a_directory_no_context_covers_is_the_book_of_nobody() {
        let contexts = vec![Context::from(("harbour", Language::default()))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let press = Press::from((languages, crate::infra::writer()));
        let directories = vec![PathBuf::from("crates/nowhere")];

        let books = press.try_gather(&directories);

        let path = PathBuf::from("crates/nowhere");
        assert_eq!(books, Err(Error::Directory { path }));
    }

    #[test]
    fn every_book_of_a_context_with_a_definition_is_of_at_least_one_term() {
        let language = Language::default()
            .try_define(["Treasure"], "A thing worth the search.")
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let press = Press::from((languages, crate::infra::writer()));

        let books = press.try_gather(&[]).unwrap();

        let barren = books.iter().any(|book| book.count() == 0);
        assert!(!barren);
    }

    #[test]
    fn a_book_of_no_term_at_all_is_taken_by_nobody() {
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let press = Press::from((languages, crate::infra::writer()));
        let book = Book::default();

        let response = press.try_write(&book);

        let path = PathBuf::new();
        assert_eq!(response, Err(Error::Vocabulary { path }));
    }
}
