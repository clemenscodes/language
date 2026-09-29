use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Sha1 {
    language: Language,
}

impl Layered for Sha1 {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Sha1 {}

impl Sha1 {
    pub(super) const NAME: &'static str = "sha1";

    const WORDS: &'static [&'static str] = &["Sha1", "Digest", "update", "finalize"];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Sha1 {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
