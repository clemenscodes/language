use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Aes {
    language: Language,
}

impl Layered for Aes {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Aes {}

impl Aes {
    pub(super) const NAME: &'static str = "aes";

    const WORDS: &'static [&'static str] = &[
        "Aes",
        "Aes128",
        "Aes256",
        "cipher",
        "Cipher",
        "Block",
        "Stream",
        "keystream",
        "Key",
        "Iv",
        "Init",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Aes {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
