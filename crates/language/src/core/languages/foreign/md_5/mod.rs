use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Md5 {
    language: Language,
}

impl Layered for Md5 {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Md5 {}

impl Md5 {
    pub(super) const NAME: &'static str = "md-5";

    const WORDS: &'static [&'static str] = &["Md5", "Digest", "update", "finalize"];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Md5 {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
