use crate::core::Language;
use crate::core::language::Error;
use ddd::{AntiCorruptionLayer, DomainLayer, Layered};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Futures {
    language: Language,
}

impl Layered for Futures {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Futures {}

impl Futures {
    pub(super) const NAME: &'static str = "futures";

    const WORDS: &'static [&'static str] = &[
        "futures",
        "Stream",
        "Future",
        "Pin",
        "Send",
        "Item",
        "Unpin",
        "Allow",
        "Ext",
        "oneshot",
        "Canceled",
        "mpsc",
        "unbounded",
    ];

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<()> for Futures {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let words = Self::WORDS.iter().copied();
        let language = Language::default().try_admit(words)?;
        Ok(Self { language })
    }
}
