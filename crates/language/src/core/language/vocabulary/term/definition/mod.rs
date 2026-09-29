pub(crate) mod error;

pub(crate) use error::Error;

use ddd::{DomainLayer, Layered, ValueObject};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Definition {
    text: String,
}

impl Layered for Definition {
    type Layer = DomainLayer;
}

impl ValueObject for Definition {}

impl TryFrom<&str> for Definition {
    type Error = Error;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let bare_text = text.trim();
        if bare_text.is_empty() {
            let text = text.to_string();
            return Err(Error::Definition { text });
        }
        let text = bare_text.to_string();
        Ok(Self { text })
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.text, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_text_yields_the_same_definition() {
        let text = "The object a team brings to the chest.";

        let first_error = Definition::try_from(text).unwrap();
        let second_error = Definition::try_from(text).unwrap();

        assert_eq!(first_error, second_error);
    }

    #[test]
    fn a_definition_drops_the_padding_around_its_text() {
        let padded_definition =
            Definition::try_from("  The object a team brings to the chest.  ").unwrap();

        let bare_text = Definition::try_from("The object a team brings to the chest.").unwrap();

        assert_eq!(padded_definition, bare_text);
    }

    #[test]
    fn two_different_texts_yield_two_different_definitions() {
        let treasure_definition =
            Definition::try_from("The object a team brings to the chest.").unwrap();

        let chest_definition = Definition::try_from("The chest a team delivers to.").unwrap();

        assert_ne!(treasure_definition, chest_definition);
    }

    #[test]
    fn a_definition_is_shown_as_its_text() {
        let definition = Definition::try_from("The object a team brings to the chest.").unwrap();

        let definition_text = definition.to_string();

        assert_eq!(definition_text, "The object a team brings to the chest.");
    }

    #[test]
    fn nothing_is_not_a_definition() {
        let text = "";

        let definition = Definition::try_from(text);

        let text = String::new();
        assert_eq!(definition, Err(Error::Definition { text }));
    }

    #[test]
    fn padding_alone_is_not_a_definition() {
        let text = "   ";

        let definition = Definition::try_from(text);

        let text = String::from("   ");
        assert_eq!(definition, Err(Error::Definition { text }));
    }
}
