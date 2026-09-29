use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Hmac {
    language: Language,
}

impl Layered for Hmac {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Hmac {}

impl Hmac {
    pub(super) const NAME: &'static str = "hmac";

    const WORDS: &'static [&'static str] = &["Hmac", "Mac", "Init", "update", "finalize"];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Hmac {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
