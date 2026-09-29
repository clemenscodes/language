use crate::core::Language;
use ddd::{DomainLayer, Layered, ValueObject};
use std::path::{Component, Path};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    root: String,
    directory_name: String,
    language: Language,
    foreign: Vec<String>,
}

impl Layered for Context {
    type Layer = DomainLayer;
}

impl ValueObject for Context {}

impl Context {
    const CRATES: &'static str = "crates";
    const UNDER: &'static str = "/";

    pub(crate) fn language(&self) -> &Language {
        &self.language
    }

    pub(crate) fn foreign(&self) -> &[String] {
        &self.foreign
    }

    pub(crate) fn root(&self) -> &str {
        &self.root
    }

    pub(crate) fn directory(&self) -> &str {
        &self.directory_name
    }

    pub(crate) fn covers(&self, path: &Path) -> bool {
        let directory_name = Self::directory_name(path, &self.root, self.depth());
        directory_name.as_deref() == Some(self.directory_name.as_str())
    }

    fn depth(&self) -> usize {
        self.directory_name.split(Self::UNDER).count()
    }

    fn directory_name(path: &Path, root: &str, depth: usize) -> Option<String> {
        let components: Vec<Component<'_>> = path.components().collect();
        let root_position = components
            .iter()
            .rposition(|component| component.as_os_str() == root)?;
        let mut directory_names = Vec::with_capacity(depth);
        for step in 1..=depth {
            let component = components.get(root_position + step)?;
            match component {
                Component::Normal(folder) => {
                    let directory_name = folder.to_string_lossy().to_string();
                    directory_names.push(directory_name);
                }
                _ => return None,
            }
        }
        let directory_name = directory_names.join(Self::UNDER);
        Some(directory_name)
    }
}

impl From<(&str, Language)> for Context {
    fn from((directory_name, language): (&str, Language)) -> Self {
        let crates = Self::CRATES;
        Self::from((crates, directory_name, language))
    }
}

impl From<(&str, &str, Language)> for Context {
    fn from((root, directory_name, language): (&str, &str, Language)) -> Self {
        let root = root.to_string();
        let directory_name = directory_name.to_string();
        let foreign = Vec::new();
        Self {
            root,
            directory_name,
            language,
            foreign,
        }
    }
}

impl From<(Context, Vec<String>)> for Context {
    fn from((context, names): (Context, Vec<String>)) -> Self {
        let mut foreign = context.foreign;
        foreign.extend(names);
        Self {
            root: context.root,
            directory_name: context.directory_name,
            language: context.language,
            foreign,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_belongs_to_exactly_one_context() {
        let contexts = [
            Context::from(("harbour", Language::default())),
            Context::from(("ship", Language::default())),
        ];
        let path = Path::new("crates/harbour/src/core/dock/mod.rs");

        let count = contexts
            .iter()
            .filter(|context| context.covers(path))
            .count();

        assert_eq!(count, 1);
    }

    #[test]
    fn a_path_deep_under_a_directory_belongs_to_the_same_context_as_a_shallow_one() {
        let contexts = [
            Context::from(("harbour", Language::default())),
            Context::from(("ship", Language::default())),
        ];
        let deep = Path::new("crates/harbour/dock/public/src/lib.rs");
        let shallow = Path::new("crates/harbour/lib.rs");

        let deep_cover: Vec<bool> = contexts
            .iter()
            .map(|context| context.covers(deep))
            .collect();

        let shallow_cover: Vec<bool> = contexts
            .iter()
            .map(|context| context.covers(shallow))
            .collect();
        assert_eq!(deep_cover, shallow_cover);
    }

    #[test]
    fn a_path_under_crates_is_covered_by_the_context_of_that_root_and_that_directory() {
        let context = Context::from(("crates", "harbour", Language::default()));
        let path = Path::new("crates/harbour/src/core/dock/mod.rs");

        let verdict = context.covers(path);

        assert!(verdict);
    }

    #[test]
    fn a_path_under_packages_is_covered_by_the_context_whose_root_is_packages() {
        let context = Context::from(("packages", "lighthouse", Language::default()));
        let path = Path::new("packages/lighthouse/src/main/java/Lamp.java");

        let verdict = context.covers(path);

        assert!(verdict);
    }

    #[test]
    fn a_path_under_packages_is_covered_by_no_context_whose_root_is_crates() {
        let contexts = [
            Context::from(("harbour", Language::default())),
            Context::from(("lighthouse", Language::default())),
        ];
        let path = Path::new("packages/lighthouse/src/main/java/Lamp.java");

        let count = contexts
            .iter()
            .filter(|context| context.root() == Context::CRATES)
            .filter(|context| context.covers(path))
            .count();

        assert_eq!(count, 0);
    }

    #[test]
    fn a_context_under_a_root_covers_no_path_of_the_same_directory_under_another_root() {
        let crates_context = Context::from(("crates", "harbour", Language::default()));
        let packages_context = Context::from(("packages", "harbour", Language::default()));
        let crates_path = Path::new("crates/harbour/src/lib.rs");
        let packages_path = Path::new("packages/harbour/src/main/java/Harbour.java");

        let crates_verdict = crates_context.covers(packages_path);

        let packages_verdict = packages_context.covers(crates_path);
        assert!(!crates_verdict);
        assert!(!packages_verdict);
    }

    #[test]
    fn a_directory_of_two_segments_is_covered_under_the_root_of_its_context() {
        let context = Context::from(("ship/deck", Language::default()));
        let path = Path::new("crates/ship/deck/src/lib.rs");

        let verdict = context.covers(path);

        assert!(verdict);
    }

    #[test]
    fn the_root_of_a_context_named_by_its_directory_alone_is_crates() {
        let context = Context::from(("harbour", Language::default()));

        let root = context.root();

        assert_eq!(root, "crates");
    }

    #[test]
    fn a_path_outside_every_root_belongs_to_no_context() {
        let contexts = [
            Context::from(("harbour", Language::default())),
            Context::from(("ship", Language::default())),
        ];
        let path = Path::new("nix/flake/default.nix");

        let count = contexts
            .iter()
            .filter(|context| context.covers(path))
            .count();

        assert_eq!(count, 0);
    }

    #[test]
    fn a_path_naming_the_root_and_nothing_below_belongs_to_no_context() {
        let contexts = [
            Context::from(("harbour", Language::default())),
            Context::from(("ship", Language::default())),
        ];
        let path = Path::new("crates");

        let count = contexts
            .iter()
            .filter(|context| context.covers(path))
            .count();

        assert_eq!(count, 0);
    }

    #[test]
    fn a_path_that_steps_back_above_the_root_belongs_to_no_context() {
        let contexts = [
            Context::from(("harbour", Language::default())),
            Context::from(("ship", Language::default())),
        ];
        let path = Path::new("crates/../ship/src/lib.rs");

        let count = contexts
            .iter()
            .filter(|context| context.covers(path))
            .count();

        assert_eq!(count, 0);
    }

    #[test]
    fn a_context_with_foreign_names_keeps_the_names_it_had_and_adds_the_new_ones() {
        let context = Context::from(("harbour", Language::default()));
        let first_names = vec![String::from("clap")];
        let second_names = vec![String::from("serde")];
        let first_context = Context::from((context, first_names));

        let named_context = Context::from((first_context, second_names));

        let foreign = named_context.foreign();
        assert_eq!(foreign, ["clap", "serde"]);
    }
}
