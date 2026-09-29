use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ClapComplete {
    language: Language,
}

impl Layered for ClapComplete {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for ClapComplete {}

impl ClapComplete {
    pub(super) const NAME: &'static str = "clap_complete";

    const WORDS: &'static [&'static str] = &[
        "clap",
        "complete",
        "generate",
        "Generator",
        "Bash",
        "Fish",
        "Zsh",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for ClapComplete {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
