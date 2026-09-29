use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Serde {
    language: Language,
}

impl Layered for Serde {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Serde {}

impl Serde {
    pub(super) const NAME: &'static str = "serde";

    const WORDS: &'static [&'static str] = &[
        "serde",
        "Serialize",
        "Deserialize",
        "Serializer",
        "Deserializer",
        "serialize",
        "deserialize",
        "serializer",
        "deserializer",
        "transparent",
        "rename",
        "tag",
        "untagged",
        "bound",
        "de",
        "ser",
        "value",
        "into",
        "Into",
        "collect",
        "str",
        "Error",
        "Unexpected",
        "invalid",
        "custom",
        "Ok",
        "D",
        "S",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Serde {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
