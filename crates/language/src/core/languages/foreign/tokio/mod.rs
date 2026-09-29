use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Tokio {
    language: Language,
}

impl Layered for Tokio {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Tokio {}

impl Tokio {
    pub(super) const NAME: &'static str = "tokio";

    const WORDS: &'static [&'static str] = &["tokio", "time", "timeout"];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Tokio {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
