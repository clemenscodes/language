use crate::core::languages::context::Context;
use ddd::{DomainLayer, Layered, ValueObject};
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct Book {
    path: PathBuf,
    rows: BTreeMap<String, String>,
}

impl Layered for Book {
    type Layer = DomainLayer;
}

impl ValueObject for Book {}

impl Book {
    const DOCUMENTS: &'static str = "docs";
    const FILE_NAME: &'static str = "UBIQUITOUS_LANGUAGE.md";
    const HEADING: &'static str = "# The ubiquitous language";
    const HEAD: &'static str = "| Word | What it means |";
    const RULE: &'static str = "|---|---|";
    const SEPARATOR: char = '|';
    const KEPT_SEPARATOR: &'static str = "\\|";

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn count(&self) -> usize {
        self.rows.len()
    }

    fn row((spelling, definition): (String, String)) -> (String, String) {
        let definition = definition.replace(Self::SEPARATOR, Self::KEPT_SEPARATOR);
        (spelling, definition)
    }
}

impl From<&Context> for Book {
    fn from(context: &Context) -> Self {
        let directory = Path::new(context.root()).join(context.directory());
        Self::from((directory.as_path(), context))
    }
}

impl From<(&Path, &Context)> for Book {
    fn from((directory, context): (&Path, &Context)) -> Self {
        let path = directory.join(Self::DOCUMENTS).join(Self::FILE_NAME);
        let rows = context.language().terms().map(Self::row).collect();
        Self { path, rows }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line = Self::HEADING;
        let head = Self::HEAD;
        let rule = Self::RULE;
        let mut text = format!("{line}\n\n{head}\n{rule}\n");
        for (spelling, definition) in &self.rows {
            let row = format!("| {spelling} | {definition} |\n");
            text.push_str(&row);
        }
        fmt::Display::fmt(&text, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Language;

    #[test]
    fn the_book_of_a_context_is_under_the_directory_of_that_context() {
        let language = Language::default()
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap();

        let book = Book::from(&Context::from(("uncharted", language)));

        let book_text = book.path().display().to_string();
        assert_eq!(book_text, "crates/uncharted/docs/UBIQUITOUS_LANGUAGE.md");
    }

    #[test]
    fn the_book_of_a_context_under_another_root_is_under_that_root() {
        let language = Language::default()
            .try_define(["Cell"], "The processor the disassembler reads.")
            .unwrap();

        let book = Book::from(&Context::from(("packages", "ghidra-cell", language)));

        let book_text = book.path().display().to_string();
        assert_eq!(
            book_text,
            "packages/ghidra-cell/docs/UBIQUITOUS_LANGUAGE.md"
        );
    }

    #[test]
    fn a_book_is_the_heading_the_head_and_one_row_for_every_term() {
        let language = Language::default()
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap();

        let book = Book::from(&Context::from(("uncharted", language)));

        let book_text = book.to_string();
        assert_eq!(
            book_text,
            concat!(
                "# The ubiquitous language\n",
                "\n",
                "| Word | What it means |\n",
                "|---|---|\n",
                "| Treasure | The object a team brings to the chest. |\n",
            )
        );
    }

    #[test]
    fn a_term_of_several_words_is_one_row_that_keeps_one_space() {
        let language = Language::default()
            .try_define(["King", "of", "the", "Hill"], "The matchtype with a hill.")
            .unwrap();

        let book = Book::from(&Context::from(("uncharted", language)));

        let book_text = book.to_string();
        assert!(book_text.ends_with("| King of the Hill | The matchtype with a hill. |\n"));
    }

    #[test]
    fn the_rows_of_a_book_are_ordered_by_the_term_they_are_of() {
        let language = Language::default()
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap()
            .try_define(["Chest"], "The chest a team delivers to.")
            .unwrap();

        let book = Book::from(&Context::from(("uncharted", language)));

        let book_text = book.to_string();
        let rows: Vec<&str> = book_text.lines().skip(4).collect();
        assert_eq!(
            rows,
            vec![
                "| Chest | The chest a team delivers to. |",
                "| Treasure | The object a team brings to the chest. |",
            ]
        );
    }

    #[test]
    fn a_separator_inside_a_definition_does_not_break_the_row_it_is_in() {
        let language = Language::default()
            .try_define(["Ticket"], "The stake, one | of the chest.")
            .unwrap();

        let book = Book::from(&Context::from(("uncharted", language)));

        let book_text = book.to_string();
        assert!(book_text.ends_with("| Ticket | The stake, one \\| of the chest. |\n"));
    }

    #[test]
    fn a_word_admitted_without_a_definition_is_in_no_row() {
        let language = Language::default().try_admit(["helper"]).unwrap();

        let book = Book::from(&Context::from(("uncharted", language)));

        let count = book.count();
        assert_eq!(count, 0);
    }

    #[test]
    fn the_book_of_a_language_without_a_term_is_a_table_of_no_row() {
        let book = Book::from(&Context::from(("uncharted", Language::default())));

        let book_text = book.to_string();

        assert_eq!(
            book_text,
            concat!(
                "# The ubiquitous language\n",
                "\n",
                "| Word | What it means |\n",
                "|---|---|\n",
            )
        );
    }

    #[test]
    fn a_book_counts_the_terms_it_comes_from() {
        let language = Language::default()
            .try_define(["Treasure"], "The object a team brings to the chest.")
            .unwrap()
            .try_define(["Chest"], "The chest a team delivers to.")
            .unwrap();

        let book = Book::from(&Context::from(("uncharted", language)));

        let count = book.count();
        assert_eq!(count, 2);
    }
}
