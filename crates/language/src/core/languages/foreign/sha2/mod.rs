use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Sha2 {
    language: Language,
}

impl Layered for Sha2 {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Sha2 {}

impl Sha2 {
    pub(super) const NAME: &'static str = "sha2";

    const WORDS: &'static [&'static str] = &["Sha2", "Sha256", "Digest", "update", "finalize"];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Sha2 {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
