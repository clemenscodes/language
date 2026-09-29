pub(crate) mod error;

pub(crate) use error::Error;

use crate::core::Dependencies;
use crate::core::Language;
use crate::core::Languages;
use crate::core::Reader;
use ddd::{ApplicationLayer, ApplicationService, Layered};
use std::path::{Path, PathBuf};

pub(crate) struct Gate {
    languages: Languages,
    reader: Box<dyn Reader>,
}

impl Layered for Gate {
    type Layer = ApplicationLayer;
}

impl ApplicationService for Gate {}

impl Gate {
    const MANIFEST_NAME: &'static str = "Cargo.toml";

    pub(crate) fn try_judge(&self, path: &Path) -> Result<(), Error> {
        let text = match self.reader.try_read(path) {
            Ok(text) => text,
            Err(cause) => {
                let path = path.to_path_buf();
                let cause = cause.to_string();
                return Err(Error::Content { path, cause });
            }
        };
        self.try_judge_text(&text, path)
    }

    fn try_judge_text(&self, text: &str, path: &Path) -> Result<(), Error> {
        let language = self.try_language(path)?;
        let unknown_vocabulary = language.unknown_vocabulary(text);
        if unknown_vocabulary.is_empty() {
            return Ok(());
        }
        let path = path.to_path_buf();
        let unknown_words = unknown_vocabulary.to_string();
        Err(Error::Language {
            path,
            unknown_words,
        })
    }

    fn try_language(&self, path: &Path) -> Result<Language, Error> {
        let shared_language = self.languages.shared_language();
        let context = self
            .languages
            .contexts()
            .iter()
            .find(|context| context.covers(path));
        let mut language = match context {
            Some(context) => {
                let own_language = context.language();
                let own_foreign = self
                    .languages
                    .foreign()
                    .iter()
                    .find(|foreign| foreign.is_named(context.directory()));
                let mut own = match own_foreign {
                    Some(own_foreign) => &(shared_language | own_language) | own_foreign.language(),
                    None => shared_language | own_language,
                };
                for name in context.foreign() {
                    for foreign in self.languages.foreign() {
                        if foreign.is_named(name) {
                            own = &own | foreign.language();
                        }
                    }
                }
                own
            }
            None => shared_language.clone(),
        };
        let dependencies = self.try_dependencies(path)?;
        for crate_path in dependencies.paths() {
            for context in self.languages.contexts() {
                if context.covers(crate_path) {
                    language = &language | context.language();
                }
            }
        }
        for crate_name in &dependencies {
            let foreign = self
                .languages
                .foreign()
                .iter()
                .find(|foreign| foreign.is_named(crate_name));
            if let Some(foreign) = foreign {
                language = &language | foreign.language();
            }
        }
        Ok(language)
    }

    fn try_dependencies(&self, path: &Path) -> Result<Dependencies, Error> {
        let manifest_path = Self::manifest_path(path);
        let Some(manifest_path) = manifest_path else {
            return Ok(Dependencies::default());
        };
        let manifest = self.try_manifest(&manifest_path)?;
        let root_manifest_path = manifest_path.parent().and_then(Self::manifest_path);
        let root_manifest = match root_manifest_path {
            Some(root_manifest_path) => self.try_manifest(&root_manifest_path)?,
            None => String::default(),
        };
        Ok(Dependencies::from((
            manifest.as_str(),
            root_manifest.as_str(),
        )))
    }

    fn try_manifest(&self, manifest_path: &Path) -> Result<String, Error> {
        match self.reader.try_read(manifest_path) {
            Ok(text) => Ok(text),
            Err(cause) => {
                let path = manifest_path.to_path_buf();
                let cause = cause.to_string();
                Err(Error::Content { path, cause })
            }
        }
    }

    fn manifest_path(path: &Path) -> Option<PathBuf> {
        let mut directory = path.parent()?;
        loop {
            let manifest_path = directory.join(Self::MANIFEST_NAME);
            if manifest_path.is_file() {
                return Some(manifest_path);
            }
            directory = directory.parent()?;
        }
    }
}

impl From<(Languages, Box<dyn Reader>)> for Gate {
    fn from((languages, reader): (Languages, Box<dyn Reader>)) -> Self {
        Self { languages, reader }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Context;
    use crate::core::Foreign;

    #[test]
    fn a_word_of_a_crate_the_root_manifest_leads_to_is_let_through() {
        struct Manifests;
        impl Layered for Manifests {
            type Layer = ddd::InfrastructureLayer;
        }
        impl ddd::Port for Manifests {}
        impl ddd::DrivenPort for Manifests {}
        impl Reader for Manifests {
            fn try_read(&self, path: &Path) -> std::io::Result<String> {
                match path.starts_with(env!("CARGO_MANIFEST_DIR")) {
                    true => Ok(String::from("[dependencies]\nharbour.workspace = true\n")),
                    false => Ok(String::from(
                        "[workspace.dependencies]\nharbour = { path = \"crates/harbour\" }\n",
                    )),
                }
            }
        }
        let library_languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let shared_language = library_languages.shared_language().clone();
        let language = Language::default().try_admit(["Plunderflux"]).unwrap();
        let context = Context::from(("harbour", language));
        let languages = Languages::from((shared_language, vec![context], Vec::new()));
        let gate = Gate::from((languages, Box::new(Manifests) as Box<dyn Reader>));
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

        let verdict = gate.try_judge_text("struct Plunderflux;", path);

        assert_eq!(verdict, Ok(()));
    }

    #[test]
    fn a_crate_under_one_directory_is_no_crate_of_the_directory_beside_it() {
        struct Manifest;
        impl Layered for Manifest {
            type Layer = ddd::InfrastructureLayer;
        }
        impl ddd::Port for Manifest {}
        impl ddd::DrivenPort for Manifest {}
        impl Reader for Manifest {
            fn try_read(&self, _: &Path) -> std::io::Result<String> {
                Ok(String::from(
                    "[dependencies]\nharbour = { path = \"crates/harbour/dock\" }\n",
                ))
            }
        }
        let library_languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let shared_language = library_languages.shared_language().clone();
        let language = Language::default().try_admit(["Plunderflux"]).unwrap();
        let context = Context::from(("harbour/quay", language));
        let languages = Languages::from((shared_language, vec![context], Vec::new()));
        let gate = Gate::from((languages, Box::new(Manifest) as Box<dyn Reader>));
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

        let verdict = gate.try_judge_text("struct Plunderflux;", path);

        assert!(verdict.is_err());
    }

    #[test]
    fn a_source_the_reader_hands_over_is_judged_by_the_language_of_its_context() {
        struct Source;
        impl Layered for Source {
            type Layer = ddd::InfrastructureLayer;
        }
        impl ddd::Port for Source {}
        impl ddd::DrivenPort for Source {}
        impl Reader for Source {
            fn try_read(&self, _: &Path) -> std::io::Result<String> {
                Ok(String::from("struct Treasure { chest: Chest }"))
            }
        }
        let language = Language::default()
            .try_admit(["Treasure", "Chest"])
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let gate = Gate::from((languages, Box::new(Source) as Box<dyn Reader>));
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/harbour/src/lib.rs"
        ));

        let verdict = gate.try_judge(path);

        assert_eq!(verdict, Ok(()));
    }

    #[test]
    fn a_source_outside_every_manifest_is_judged_by_the_shared_language_alone() {
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let path = Path::new("/nowhere/lib.rs");

        let verdict = gate.try_judge_text("struct Vec { len: usize }", path);

        assert_eq!(verdict, Ok(()));
    }

    #[test]
    fn a_source_at_the_root_of_everything_has_no_directory_above_it() {
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let path = Path::new("/");

        let verdict = gate.try_judge_text("struct Vec { len: usize }", path);

        assert_eq!(verdict, Ok(()));
    }

    #[test]
    fn a_gate_of_no_context_at_all_judges_by_the_shared_language_it_was_handed() {
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/harbour/src/lib.rs"
        ));

        let verdict = gate.try_judge_text("struct Treasure { chest: Chest }", path);

        assert!(verdict.is_err());
    }

    #[test]
    fn a_manifest_the_reader_cannot_read_leaves_the_source_unjudged() {
        struct Shut;
        impl Layered for Shut {
            type Layer = ddd::InfrastructureLayer;
        }
        impl ddd::Port for Shut {}
        impl ddd::DrivenPort for Shut {}
        impl Reader for Shut {
            fn try_read(&self, path: &Path) -> std::io::Result<String> {
                match path.ends_with(Gate::MANIFEST_NAME) {
                    true => Err(std::io::Error::other("shut")),
                    false => Ok(String::from("struct Vec { len: usize }")),
                }
            }
        }
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let gate = Gate::from((languages, Box::new(Shut) as Box<dyn Reader>));
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/core/language/vocabulary/word/mod.rs"
        ));

        let verdict = gate.try_judge(path);

        assert!(verdict.is_err());
    }

    #[test]
    fn a_root_manifest_the_reader_cannot_read_leaves_the_source_unjudged() {
        struct Crate;
        impl Layered for Crate {
            type Layer = ddd::InfrastructureLayer;
        }
        impl ddd::Port for Crate {}
        impl ddd::DrivenPort for Crate {}
        impl Reader for Crate {
            fn try_read(&self, path: &Path) -> std::io::Result<String> {
                match path.starts_with(env!("CARGO_MANIFEST_DIR")) {
                    true => Ok(String::default()),
                    false => Err(std::io::Error::other("shut")),
                }
            }
        }
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let gate = Gate::from((languages, Box::new(Crate) as Box<dyn Reader>));
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

        let verdict = gate.try_judge_text("struct Vec { len: usize }", path);

        assert!(verdict.is_err());
    }

    #[test]
    fn a_source_beside_the_root_manifest_has_no_manifest_above_it() {
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let crate_directory = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root_directory = crate_directory.parent().unwrap().parent().unwrap();
        let path = root_directory.join("lib.rs");

        let verdict = gate.try_judge_text("struct Vec { len: usize }", &path);

        assert_eq!(verdict, Ok(()));
    }

    #[test]
    fn a_source_in_the_language_of_its_context_is_let_through() {
        let language = Language::default()
            .try_admit(["Treasure", "Chest"])
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/harbour/src/lib.rs"
        ));

        let verdict = gate.try_judge_text("struct Treasure { chest: Chest }", path);

        assert_eq!(verdict, Ok(()));
    }

    #[test]
    fn a_source_with_a_word_of_no_language_is_an_error() {
        let language = Language::default()
            .try_admit(["Treasure", "Chest"])
            .unwrap();
        let contexts = vec![Context::from(("harbour", language))];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/harbour/src/lib.rs"
        ));

        let verdict = gate.try_judge_text("struct Zonk { flarn: Kwib }", path);

        let error = verdict.unwrap_err();
        let error_text = error.to_string();
        let path_text = path.display();
        let expectation = format!("[LANGUAGE] {path_text} writes flarn, Kwib, Zonk.");
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_word_of_one_context_is_no_word_of_another() {
        let harbour_language = Language::default().try_admit(["Treasure"]).unwrap();
        let contexts = vec![
            Context::from(("harbour", harbour_language)),
            Context::from(("ship", Language::default())),
        ];
        let languages = Languages::try_from((contexts, Vec::new())).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let harbour_path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/harbour/src/lib.rs"
        ));
        let ship_path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/ship/src/lib.rs"
        ));
        let harbour_verdict = gate.try_judge_text("struct Treasure;", harbour_path);

        let verdict = gate.try_judge_text("struct Treasure;", ship_path);

        assert_eq!(harbour_verdict, Ok(()));
        assert!(verdict.is_err());
    }

    #[test]
    fn a_word_of_a_crate_the_manifest_names_is_let_through() {
        struct Manifest;
        impl Layered for Manifest {
            type Layer = ddd::InfrastructureLayer;
        }
        impl ddd::Port for Manifest {}
        impl ddd::DrivenPort for Manifest {}
        impl Reader for Manifest {
            fn try_read(&self, _: &Path) -> std::io::Result<String> {
                Ok(String::from("[dependencies]\nblowfish.workspace = true\n"))
            }
        }
        let blowfish = Language::default().try_admit(["Blowfish"]).unwrap();
        let own_foreign = vec![Foreign::from(("blowfish", blowfish))];
        let languages = Languages::try_from((Vec::new(), own_foreign)).unwrap();
        let gate = Gate::from((languages, Box::new(Manifest) as Box<dyn Reader>));
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

        let verdict = gate.try_judge_text("struct Blowfish;", path);

        assert_eq!(verdict, Ok(()));
    }

    #[test]
    fn a_word_of_a_crate_no_manifest_names_is_an_error() {
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/harbour/src/lib.rs"
        ));

        let verdict = gate.try_judge_text("struct Blowfish;", path);

        assert!(verdict.is_err());
    }

    #[test]
    fn a_gate_of_no_foreign_language_makes_a_word_of_a_named_crate_an_error() {
        struct Manifest;
        impl Layered for Manifest {
            type Layer = ddd::InfrastructureLayer;
        }
        impl ddd::Port for Manifest {}
        impl ddd::DrivenPort for Manifest {}
        impl Reader for Manifest {
            fn try_read(&self, _: &Path) -> std::io::Result<String> {
                Ok(String::from("[dependencies]\nblowfish.workspace = true\n"))
            }
        }
        let library_languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let shared_language = library_languages.shared_language().clone();
        let languages = Languages::from((shared_language, Vec::new(), Vec::new()));
        let gate = Gate::from((languages, Box::new(Manifest) as Box<dyn Reader>));
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

        let verdict = gate.try_judge_text("struct Blowfish;", path);

        assert!(verdict.is_err());
    }

    #[test]
    fn a_word_of_a_foreign_language_the_context_names_is_let_through() {
        let lighthouse = Language::default().try_admit(["Lamp"]).unwrap();
        let own_foreign = vec![Foreign::from(("lighthouse", lighthouse))];
        let names = vec![String::from("lighthouse")];
        let harbour = Context::from(("harbour", Language::default()));
        let contexts = vec![Context::from((harbour, names))];
        let languages = Languages::try_from((contexts, own_foreign)).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/harbour/src/lib.rs"
        ));

        let verdict = gate.try_judge_text("struct Lamp;", path);

        assert_eq!(verdict, Ok(()));
    }

    #[test]
    fn a_word_of_a_foreign_language_no_context_names_is_an_error() {
        let lighthouse = Language::default().try_admit(["Lamp"]).unwrap();
        let own_foreign = vec![Foreign::from(("lighthouse", lighthouse))];
        let contexts = vec![Context::from(("harbour", Language::default()))];
        let languages = Languages::try_from((contexts, own_foreign)).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/harbour/src/lib.rs"
        ));

        let verdict = gate.try_judge_text("struct Lamp;", path);

        assert!(verdict.is_err());
    }

    #[test]
    fn a_source_that_could_not_be_read_names_the_path_and_the_cause() {
        let languages = Languages::try_from((Vec::new(), Vec::new())).unwrap();
        let gate = Gate::from((languages, crate::infra::reader()));
        let path = Path::new("nowhere/mod.rs");

        let verdict = gate.try_judge(path);

        let error = verdict.unwrap_err();
        let error_text = error.to_string();
        assert!(error_text.starts_with("[LANGUAGE] nowhere/mod.rs could not be read,"));
    }
}
