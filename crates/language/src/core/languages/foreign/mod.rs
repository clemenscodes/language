mod aes;
mod blowfish;
mod cbc;
mod clap;
mod clap_complete;
mod cmac;
mod console;
mod ctr;
mod ddd;
mod dialoguer;
mod flate2;
mod futures;
mod hmac;
mod indicatif;
mod md_5;
mod miniz_oxide;
mod serde;
mod serde_json;
mod sha1;
mod sha2;
mod tokio;

use crate::core::Language;
use crate::core::language::Error;
use crate::core::languages::foreign::aes::Aes;
use crate::core::languages::foreign::blowfish::Blowfish;
use crate::core::languages::foreign::cbc::Cbc;
use crate::core::languages::foreign::clap::Clap;
use crate::core::languages::foreign::clap_complete::ClapComplete;
use crate::core::languages::foreign::cmac::Cmac;
use crate::core::languages::foreign::console::Console;
use crate::core::languages::foreign::ctr::Ctr;
use crate::core::languages::foreign::ddd::Ddd;
use crate::core::languages::foreign::dialoguer::Dialoguer;
use crate::core::languages::foreign::flate2::Flate2;
use crate::core::languages::foreign::futures::Futures;
use crate::core::languages::foreign::hmac::Hmac;
use crate::core::languages::foreign::indicatif::Indicatif;
use crate::core::languages::foreign::md_5::Md5;
use crate::core::languages::foreign::miniz_oxide::MinizOxide;
use crate::core::languages::foreign::serde::Serde;
use crate::core::languages::foreign::serde_json::SerdeJson;
use crate::core::languages::foreign::sha1::Sha1;
use crate::core::languages::foreign::sha2::Sha2;
use crate::core::languages::foreign::tokio::Tokio;
use ::ddd::{DomainLayer, Layered, ValueObject};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Foreign {
    name: String,
    language: Language,
}

impl Layered for Foreign {
    type Layer = DomainLayer;
}

impl ValueObject for Foreign {}

impl Foreign {
    pub(crate) fn try_gather() -> Result<Vec<Self>, Error> {
        let aes = Aes::try_from(())?;
        let blowfish = Blowfish::try_from(())?;
        let cbc = Cbc::try_from(())?;
        let clap = Clap::try_from(())?;
        let clap_complete = ClapComplete::try_from(())?;
        let cmac = Cmac::try_from(())?;
        let console = Console::try_from(())?;
        let ctr = Ctr::try_from(())?;
        let ddd = Ddd::try_from(())?;
        let dialoguer = Dialoguer::try_from(())?;
        let flate2 = Flate2::try_from(())?;
        let futures = Futures::try_from(())?;
        let hmac = Hmac::try_from(())?;
        let indicatif = Indicatif::try_from(())?;
        let md5 = Md5::try_from(())?;
        let miniz = MinizOxide::try_from(())?;
        let serde = Serde::try_from(())?;
        let serde_json = SerdeJson::try_from(())?;
        let sha1 = Sha1::try_from(())?;
        let sha2 = Sha2::try_from(())?;
        let tokio = Tokio::try_from(())?;
        let foreign = vec![
            Self::from((Aes::NAME, aes.language().clone())),
            Self::from((Blowfish::NAME, blowfish.language().clone())),
            Self::from((Cbc::NAME, cbc.language().clone())),
            Self::from((Clap::NAME, clap.language().clone())),
            Self::from((ClapComplete::NAME, clap_complete.language().clone())),
            Self::from((Cmac::NAME, cmac.language().clone())),
            Self::from((Console::NAME, console.language().clone())),
            Self::from((Ctr::NAME, ctr.language().clone())),
            Self::from((Ddd::NAME, ddd.language().clone())),
            Self::from((Dialoguer::NAME, dialoguer.language().clone())),
            Self::from((Flate2::NAME, flate2.language().clone())),
            Self::from((Futures::NAME, futures.language().clone())),
            Self::from((Hmac::NAME, hmac.language().clone())),
            Self::from((Indicatif::NAME, indicatif.language().clone())),
            Self::from((Md5::NAME, md5.language().clone())),
            Self::from((MinizOxide::NAME, miniz.language().clone())),
            Self::from((Serde::NAME, serde.language().clone())),
            Self::from((SerdeJson::NAME, serde_json.language().clone())),
            Self::from((Sha1::NAME, sha1.language().clone())),
            Self::from((Sha2::NAME, sha2.language().clone())),
            Self::from((Tokio::NAME, tokio.language().clone())),
        ];
        Ok(foreign)
    }

    pub(crate) fn language(&self) -> &Language {
        &self.language
    }

    pub(crate) fn is_named(&self, name: &str) -> bool {
        self.name == name
    }
}

impl From<(&str, Language)> for Foreign {
    fn from((name, language): (&str, Language)) -> Self {
        let name = name.to_string();
        Self { name, language }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_this_library_adopts_from_outside_has_a_name_of_its_own() {
        let foreign = Foreign::try_gather().unwrap();

        let count = foreign.len();

        assert_eq!(count, 21);
    }

    #[test]
    fn a_language_is_found_by_the_name_the_manifest_contains() {
        let foreign = Foreign::try_gather().unwrap();

        let found_language = foreign
            .iter()
            .filter(|foreign| foreign.is_named("blowfish"))
            .count();

        assert_eq!(found_language, 1);
    }

    #[test]
    fn a_crate_nobody_defined_a_language_for_is_found_by_nobody() {
        let foreign = Foreign::try_gather().unwrap();

        let found_language = foreign
            .iter()
            .filter(|foreign| foreign.is_named("libc"))
            .count();

        assert_eq!(found_language, 0);
    }

    #[test]
    fn the_language_of_a_third_party_crate_lets_its_own_words_through() {
        let foreign = Foreign::try_gather().unwrap();
        let blowfish = foreign
            .iter()
            .find(|foreign| foreign.is_named("blowfish"))
            .unwrap();

        let unknown_vocabulary = blowfish.language().unknown_vocabulary("Blowfish");

        assert!(unknown_vocabulary.is_empty());
    }
}
