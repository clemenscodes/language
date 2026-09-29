use ddd::{DomainLayer, Layered, ValueObject};
use std::path::PathBuf;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct Dependencies {
    crate_names: Vec<String>,
    paths: Vec<PathBuf>,
}

impl Layered for Dependencies {
    type Layer = DomainLayer;
}

impl ValueObject for Dependencies {}

impl Dependencies {
    const OPENING: &'static str = "[dependencies]";

    const ROOT_OPENING: &'static str = "[workspace.dependencies]";

    const SECTION: char = '[';

    const PATH: &'static str = "path";

    pub(crate) fn paths(&self) -> Iter<'_, PathBuf> {
        self.paths.iter()
    }

    fn lines<'manifest>(manifest: &'manifest str, opening: &str) -> Vec<&'manifest str> {
        let mut lines = Vec::new();
        let mut inside_section = false;
        for line in manifest.lines() {
            let bare_line = line.trim();
            if bare_line == opening {
                inside_section = true;
                continue;
            }
            if bare_line.starts_with(Self::SECTION) {
                inside_section = false;
                continue;
            }
            if !inside_section {
                continue;
            }
            lines.push(bare_line);
        }
        lines
    }

    fn crate_name(line: &str) -> Option<&str> {
        let (left, _) = line.split_once('=')?;
        let crate_name = left.trim().split('.').next().unwrap_or(left).trim();
        match crate_name.is_empty() {
            true => None,
            false => Some(crate_name),
        }
    }

    fn path(line: &str) -> Option<PathBuf> {
        let (_, right) = line.split_once('=')?;
        let entries = right.trim().trim_matches(['{', '}']);
        for entry in entries.split(',') {
            let Some((name, value)) = entry.split_once('=') else {
                continue;
            };
            if name.trim() != Self::PATH {
                continue;
            }
            let path = value.trim().trim_matches('"');
            return Some(PathBuf::from(path));
        }
        None
    }

    fn root_path(root_manifest: &str, crate_name: &str) -> Option<PathBuf> {
        let lines = Self::lines(root_manifest, Self::ROOT_OPENING);
        for line in lines {
            if Self::crate_name(line) == Some(crate_name) {
                return Self::path(line);
            }
        }
        None
    }
}

impl From<(&str, &str)> for Dependencies {
    fn from((manifest, root_manifest): (&str, &str)) -> Self {
        let mut crate_names = Vec::new();
        let mut paths = Vec::new();
        let lines = Self::lines(manifest, Self::OPENING);
        for line in lines {
            let Some(crate_name) = Self::crate_name(line) else {
                continue;
            };
            let own_path = Self::path(line);
            let path = match own_path {
                Some(own_path) => Some(own_path),
                None => Self::root_path(root_manifest, crate_name),
            };
            if let Some(path) = path {
                paths.push(path);
            }
            crate_names.push(crate_name.to_string());
        }
        Self { crate_names, paths }
    }
}

impl<'dependencies> IntoIterator for &'dependencies Dependencies {
    type Item = &'dependencies String;
    type IntoIter = Iter<'dependencies, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.crate_names.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn a_line_of_a_dependency_without_a_name_names_no_crate() {
        let manifest = "[dependencies]\n = true\nhmac = \"=0.13.0\"\n";

        let dependencies = Dependencies::from((manifest, ""));

        let crate_names: Vec<&String> = dependencies.into_iter().collect();
        assert_eq!(crate_names, vec!["hmac"]);
    }

    #[test]
    fn a_manifest_names_the_crates_its_dependencies_section_lists() {
        let manifest = "[package]\nname = \"one\"\n\n[dependencies]\nblowfish.workspace = true\nhmac = \"=0.13.0\"\n";

        let dependencies = Dependencies::from((manifest, ""));

        let crate_names: Vec<&String> = dependencies.into_iter().collect();
        assert_eq!(crate_names, vec!["blowfish", "hmac"]);
    }

    #[test]
    fn a_section_after_the_dependencies_is_no_dependency() {
        let manifest =
            "[dependencies]\nblowfish.workspace = true\n\n[dev-dependencies]\ntempfile = \"1\"\n";

        let dependencies = Dependencies::from((manifest, ""));

        let crate_names: Vec<&String> = dependencies.into_iter().collect();
        assert_eq!(crate_names, vec!["blowfish"]);
    }

    #[test]
    fn a_manifest_without_a_dependencies_section_names_nothing() {
        let manifest = "[package]\nname = \"one\"\n";

        let dependencies = Dependencies::from((manifest, ""));

        let crate_names: Vec<&String> = dependencies.into_iter().collect();
        assert!(crate_names.is_empty());
    }

    #[test]
    fn a_renamed_dependency_is_named_the_way_the_line_opens() {
        let manifest =
            "[dependencies]\nport = { path = \"../port\", package = \"plumbing-port\" }\n";

        let dependencies = Dependencies::from((manifest, ""));

        let crate_names: Vec<&String> = dependencies.into_iter().collect();
        assert_eq!(crate_names, vec!["port"]);
    }

    #[test]
    fn a_dependency_that_writes_its_own_path_leads_to_that_path() {
        let manifest = "[dependencies]\nxbreak = { path = \"crates/xbreak/lib\" }\n";

        let dependencies = Dependencies::from((manifest, ""));

        let paths: Vec<&PathBuf> = dependencies.paths().collect();
        assert_eq!(paths, vec![Path::new("crates/xbreak/lib")]);
    }

    #[test]
    fn a_dependency_of_the_root_manifest_leads_to_the_path_the_root_manifest_writes() {
        let manifest = "[dependencies]\nxbreak.workspace = true\n";
        let root_manifest = "[workspace.dependencies]\nxbreak = { path = \"crates/xbreak/lib\" }\n";

        let dependencies = Dependencies::from((manifest, root_manifest));

        let paths: Vec<&PathBuf> = dependencies.paths().collect();
        assert_eq!(paths, vec![Path::new("crates/xbreak/lib")]);
    }

    #[test]
    fn a_dependency_the_root_manifest_writes_a_version_for_leads_nowhere() {
        let manifest = "[dependencies]\nhmac.workspace = true\n";
        let root_manifest = "[workspace.dependencies]\nhmac = \"=0.13.0\"\n";

        let dependencies = Dependencies::from((manifest, root_manifest));

        let paths: Vec<&PathBuf> = dependencies.paths().collect();
        assert!(paths.is_empty());
    }

    #[test]
    fn a_dependency_of_another_name_than_the_root_manifest_writes_leads_nowhere() {
        let manifest = "[dependencies]\nxbreak.workspace = true\n";
        let root_manifest = "[workspace.dependencies]\nsony = { path = \"crates/sony\" }\n";

        let dependencies = Dependencies::from((manifest, root_manifest));

        let paths: Vec<&PathBuf> = dependencies.paths().collect();
        assert!(paths.is_empty());
    }

    #[test]
    fn a_section_after_the_root_dependencies_names_no_path() {
        let manifest = "[dependencies]\nxbreak.workspace = true\n";
        let root_manifest = "[workspace.dependencies]\n\n[patch.crates-io]\nxbreak = { path = \"crates/xbreak/lib\" }\n";

        let dependencies = Dependencies::from((manifest, root_manifest));

        let paths: Vec<&PathBuf> = dependencies.paths().collect();
        assert!(paths.is_empty());
    }

    #[test]
    fn a_dependency_of_a_version_alone_leads_nowhere() {
        let manifest = "[dependencies]\nhmac = \"=0.13.0\"\n";

        let dependencies = Dependencies::from((manifest, ""));

        let paths: Vec<&PathBuf> = dependencies.paths().collect();
        assert!(paths.is_empty());
    }

    #[test]
    fn a_dependency_of_features_alone_leads_nowhere() {
        let manifest =
            "[dependencies]\nflate2 = { version = \"=1.1.10\", features = [\"zlib\"] }\n";

        let dependencies = Dependencies::from((manifest, ""));

        let paths: Vec<&PathBuf> = dependencies.paths().collect();
        assert!(paths.is_empty());
    }

    #[test]
    fn a_line_of_a_dependency_without_a_value_leads_nowhere() {
        let line = "xbreak";

        let path = Dependencies::path(line);

        assert!(path.is_none());
    }

    #[test]
    fn a_manifest_of_nothing_at_all_names_no_crate_and_no_path() {
        let manifest = "";

        let dependencies = Dependencies::from((manifest, ""));

        let crate_names: Vec<&String> = dependencies.into_iter().collect();
        assert!(crate_names.is_empty());
        assert!(dependencies.paths().next().is_none());
    }
}
