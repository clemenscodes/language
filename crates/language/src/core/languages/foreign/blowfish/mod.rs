use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Blowfish {
    language: Language,
}

impl Layered for Blowfish {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Blowfish {}

impl Blowfish {
    pub(super) const NAME: &'static str = "blowfish";

    const WORDS: &'static [&'static str] = &[
        "Blowfish", "Block", "Init", "cipher", "bc", "expand", "key", "state",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Blowfish {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
