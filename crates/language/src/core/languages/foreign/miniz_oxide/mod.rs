use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct MinizOxide {
    language: Language,
}

impl Layered for MinizOxide {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for MinizOxide {}

impl MinizOxide {
    pub(super) const NAME: &'static str = "miniz_oxide";

    const WORDS: &'static [&'static str] = &[
        "Miniz",
        "Oxide",
        "inflate",
        "deflate",
        "decompress",
        "compress",
        "zlib",
        "limit",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for MinizOxide {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
