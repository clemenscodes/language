use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SerdeJson {
    language: Language,
}

impl Layered for SerdeJson {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for SerdeJson {}

impl SerdeJson {
    pub(super) const NAME: &'static str = "serde_json";

    const WORDS: &'static [&'static str] = &["json", "pretty", "serde"];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for SerdeJson {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
