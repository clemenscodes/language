use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Cbc {
    language: Language,
}

impl Layered for Cbc {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Cbc {}

impl Cbc {
    pub(super) const NAME: &'static str = "cbc";

    const WORDS: &'static [&'static str] = &[
        "Cbc",
        "Decryptor",
        "cipher",
        "Block",
        "Mode",
        "Decrypt",
        "Key",
        "Iv",
        "Init",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Cbc {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
