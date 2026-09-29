use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Clap {
    language: Language,
}

impl Layered for Clap {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Clap {}

impl Clap {
    pub(super) const NAME: &'static str = "clap";

    const WORDS: &'static [&'static str] = &[
        "clap",
        "Args",
        "Subcommand",
        "Parser",
        "arg",
        "command",
        "about",
        "version",
        "after",
        "help",
        "long",
        "global",
        "t",
        "flatten",
        "Value",
        "Enum",
        "value",
        "builder",
        "required",
        "Factory",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Clap {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
