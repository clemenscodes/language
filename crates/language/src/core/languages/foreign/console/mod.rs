use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Console {
    language: Language,
}

impl Layered for Console {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Console {}

impl Console {
    pub(super) const NAME: &'static str = "console";

    const WORDS: &'static [&'static str] = &[
        "console", "Style", "Styled", "Object", "Term", "style", "colors", "enabled", "bold",
        "dim", "cyan", "green", "red", "yellow", "magenta", "blue", "white", "apply", "stderr",
        "stdout", "measure", "text", "width", "strip", "ansi", "codes", "force", "styling",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Console {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
