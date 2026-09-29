use crate::core::Language;
use crate::core::language::Error;
use ::ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Ddd {
    language: Language,
}

impl Layered for Ddd {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Ddd {}

impl Ddd {
    pub(super) const NAME: &'static str = "ddd";

    const WORDS: &'static [&'static str] = &[
        "ddd",
        "Layered",
        "Layer",
        "Domain",
        "Application",
        "Presentation",
        "Infrastructure",
        "Value",
        "Object",
        "Port",
        "Driven",
        "Adapter",
        "Command",
        "Handler",
        "Service",
        "Aggregate",
        "Root",
        "Shared",
        "Kernel",
        "Anti",
        "Corruption",
        "Specification",
        "satisfied",
        "candidate",
        "Driving",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Ddd {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
