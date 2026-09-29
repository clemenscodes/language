mod rust;

pub(crate) mod context;
pub(crate) mod foreign;

pub use context::Context;
pub use foreign::Foreign;

use crate::core::Language;
use crate::core::language::Error;
use crate::core::languages::rust::Rust;
use ddd::{DomainLayer, Layered, ValueObject};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Languages {
    shared_language: Language,
    contexts: Vec<Context>,
    foreign: Vec<Foreign>,
}

impl Layered for Languages {
    type Layer = DomainLayer;
}

impl ValueObject for Languages {}

impl Languages {
    pub(crate) fn shared_language(&self) -> &Language {
        &self.shared_language
    }

    pub(crate) fn contexts(&self) -> &[Context] {
        &self.contexts
    }

    pub(crate) fn foreign(&self) -> &[Foreign] {
        &self.foreign
    }
}

impl From<(Language, Vec<Context>, Vec<Foreign>)> for Languages {
    fn from((shared_language, contexts, foreign): (Language, Vec<Context>, Vec<Foreign>)) -> Self {
        Self {
            shared_language,
            contexts,
            foreign,
        }
    }
}

impl TryFrom<(Vec<Context>, Vec<Foreign>)> for Languages {
    type Error = Error;

    fn try_from(
        (contexts, own_foreign): (Vec<Context>, Vec<Foreign>),
    ) -> Result<Self, Self::Error> {
        let rust = Rust::try_from(())?;
        let shared_language = rust.language().clone();
        let mut foreign = Foreign::try_gather()?;
        foreign.extend(own_foreign);
        Ok(Self::from((shared_language, contexts, foreign)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages_of_no_foreign_language_of_their_own_hold_every_foreign_language_of_the_library() {
        let contexts = Vec::new();
        let own_foreign = Vec::new();

        let languages = Languages::try_from((contexts, own_foreign)).unwrap();

        let count = languages.foreign().len();
        assert_eq!(count, 21);
    }

    #[test]
    fn a_foreign_language_of_the_repository_joins_the_foreign_languages_of_the_library() {
        let contexts = Vec::new();
        let lighthouse = Language::default().try_admit(["Lamp"]).unwrap();
        let own_foreign = vec![Foreign::from(("lighthouse", lighthouse))];

        let languages = Languages::try_from((contexts, own_foreign)).unwrap();

        let named = languages
            .foreign()
            .iter()
            .filter(|foreign| foreign.is_named("lighthouse"))
            .count();
        assert_eq!(named, 1);
    }

    #[test]
    fn the_shared_language_admits_a_keyword_of_rust() {
        let contexts = Vec::new();
        let own_foreign = Vec::new();

        let languages = Languages::try_from((contexts, own_foreign)).unwrap();

        let unknown_vocabulary = languages
            .shared_language()
            .unknown_vocabulary("struct usize");
        assert!(unknown_vocabulary.is_empty());
    }

    #[test]
    fn the_shared_language_refuses_a_word_of_one_context_alone() {
        let treasure = Language::default().try_admit(["treasure"]).unwrap();
        let contexts = vec![Context::from(("harbour", treasure))];
        let own_foreign = Vec::new();

        let languages = Languages::try_from((contexts, own_foreign)).unwrap();

        let unknown_vocabulary = languages.shared_language().unknown_vocabulary("treasure");
        assert!(!unknown_vocabulary.is_empty());
    }

    #[test]
    fn languages_of_nothing_at_all_hold_no_context_and_no_foreign_language() {
        let shared = Language::default();

        let languages = Languages::from((shared, Vec::new(), Vec::new()));

        assert!(languages.contexts().is_empty());
        assert!(languages.foreign().is_empty());
    }

    #[test]
    fn the_languages_of_the_same_contexts_are_the_same_every_time() {
        let first_contexts: Vec<Context> = Vec::new();
        let second_contexts: Vec<Context> = Vec::new();
        let first_foreign: Vec<Foreign> = Vec::new();
        let second_foreign: Vec<Foreign> = Vec::new();

        let first_languages = Languages::try_from((first_contexts, first_foreign)).unwrap();
        let second_languages = Languages::try_from((second_contexts, second_foreign)).unwrap();

        assert_eq!(first_languages, second_languages);
    }
}
