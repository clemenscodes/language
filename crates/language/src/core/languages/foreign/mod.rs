use crate::core::Language;
use ::ddd::{DomainLayer, Layered, ValueObject};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Foreign {
    name: String,
    language: Language,
}

impl Layered for Foreign {
    type Layer = DomainLayer;
}

impl ValueObject for Foreign {}

impl Foreign {
    pub(crate) fn language(&self) -> &Language {
        &self.language
    }

    pub(crate) fn is_named(&self, name: &str) -> bool {
        self.name == name
    }
}

impl From<(&str, Language)> for Foreign {
    fn from((name, language): (&str, Language)) -> Self {
        let name = name.to_string();
        Self { name, language }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_language_is_found_by_the_name_the_manifest_contains() {
        let language = Language::default().try_admit(["Blowfish"]).unwrap();

        let foreign = Foreign::from(("blowfish", language));

        assert!(foreign.is_named("blowfish"));
    }

    #[test]
    fn a_language_is_found_by_no_other_name() {
        let language = Language::default().try_admit(["Blowfish"]).unwrap();

        let foreign = Foreign::from(("blowfish", language));

        assert!(!foreign.is_named("libc"));
    }

    #[test]
    fn the_language_of_a_third_party_crate_lets_its_own_words_through() {
        let language = Language::default().try_admit(["Blowfish"]).unwrap();

        let foreign = Foreign::from(("blowfish", language));

        let unknown_vocabulary = foreign.language().unknown_vocabulary("Blowfish");
        assert!(unknown_vocabulary.is_empty());
    }
}
