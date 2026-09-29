use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Cmac {
    language: Language,
}

impl Layered for Cmac {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Cmac {}

impl Cmac {
    pub(super) const NAME: &'static str = "cmac";

    const WORDS: &'static [&'static str] = &["Cmac", "Mac", "Init", "update", "verify", "slice"];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Cmac {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
