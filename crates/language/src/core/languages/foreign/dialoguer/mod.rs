use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Dialoguer {
    language: Language,
}

impl Layered for Dialoguer {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Dialoguer {}

impl Dialoguer {
    pub(super) const NAME: &'static str = "dialoguer";

    const WORDS: &'static [&'static str] = &[
        "dialoguer",
        "Input",
        "Select",
        "Multi",
        "Confirm",
        "Colorful",
        "Theme",
        "theme",
        "with",
        "prompt",
        "items",
        "item",
        "interact",
        "text",
        "opt",
        "default",
        "defaults",
        "allow",
        "empty",
        "report",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Dialoguer {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
