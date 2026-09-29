use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Indicatif {
    language: Language,
}

impl Layered for Indicatif {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Indicatif {}

impl Indicatif {
    pub(super) const NAME: &'static str = "indicatif";

    const WORDS: &'static [&'static str] = &[
        "indicatif",
        "Progress",
        "Bar",
        "Style",
        "Draw",
        "Target",
        "new",
        "spinner",
        "enable",
        "steady",
        "tick",
        "strings",
        "template",
        "set",
        "message",
        "finish",
        "and",
        "clear",
        "draw",
        "target",
        "hidden",
        "with",
        "style",
        "stderr",
        "suspend",
        "color",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Indicatif {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
