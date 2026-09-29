use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Ctr {
    language: Language,
}

impl Layered for Ctr {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Ctr {}

impl Ctr {
    pub(super) const NAME: &'static str = "ctr";

    const WORDS: &'static [&'static str] = &["Ctr", "Ctr128"];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Ctr {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
