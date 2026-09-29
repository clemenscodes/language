use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Flate2 {
    language: Language,
}

impl Layered for Flate2 {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Flate2 {}

impl Flate2 {
    pub(super) const NAME: &'static str = "flate2";

    const WORDS: &'static [&'static str] = &[
        "flate2",
        "Compression",
        "decoder",
        "encoder",
        "finish",
        "Zlib",
        "ZLIB",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Flate2 {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
