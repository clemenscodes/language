use crate::core::Reader;
use crate::core::Writer;
use ddd::{Adapter, DrivenPort, InfrastructureLayer, Layered, Port};
use std::fs::{read_to_string, remove_file, rename, write};
use std::io;
use std::path::{Path, PathBuf};
use std::process::id;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct FileSystem;

impl Layered for FileSystem {
    type Layer = InfrastructureLayer;
}

impl Adapter for FileSystem {}

impl Port for FileSystem {}

impl DrivenPort for FileSystem {}

impl FileSystem {
    const MARK: char = '.';

    fn beside(path: &Path) -> PathBuf {
        let mut name = path.as_os_str().to_os_string();
        let mark = id();
        name.push(format!("{}{mark}", Self::MARK));
        PathBuf::from(name)
    }
}

impl Reader for FileSystem {
    fn try_read(&self, path: &Path) -> io::Result<String> {
        read_to_string(path)
    }
}

impl Writer for FileSystem {
    fn try_write(&self, path: &Path, text: &str) -> io::Result<()> {
        let beside = Self::beside(path);
        write(&beside, text)?;
        match rename(&beside, path) {
            Ok(()) => Ok(()),
            Err(cause) => match remove_file(&beside) {
                Ok(()) => Err(cause),
                Err(removal) => {
                    let beside_text = beside.display();
                    let message = format!("{cause}, and {beside_text} remains, {removal}");
                    Err(io::Error::other(message))
                }
            },
        }
    }

    fn has_directory(&self, path: &Path) -> bool {
        path.parent().is_some_and(Path::is_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, read_dir, remove_dir_all};

    #[test]
    fn a_path_whose_directory_is_here_states_so_and_one_that_is_not_does_not() {
        let adapter = FileSystem;
        let source = Path::new("src/main.rs");
        let nowhere = Path::new("nowhere/main.rs");

        let present = adapter.has_directory(source);
        let absent = adapter.has_directory(nowhere);

        assert!(present);
        assert!(!absent);
    }

    #[test]
    fn a_path_that_is_not_here_is_read_by_nobody() {
        let adapter = FileSystem;
        let path = Path::new("nowhere/mod.rs");

        let text = adapter.try_read(path);

        assert!(text.is_err());
    }

    #[test]
    fn a_path_whose_directory_is_not_there_is_taken_by_nobody() {
        let adapter = FileSystem;
        let path = Path::new("nowhere/mod.rs");

        let response = adapter.try_write(path, "one");

        let error = response.unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn what_the_file_system_wrote_is_what_it_reads_back() {
        let adapter = FileSystem;
        let directory = std::env::temp_dir().join("language-once");
        remove_dir_all(&directory).ok();
        create_dir_all(&directory).unwrap();
        let path = directory.join("mod.rs");

        adapter.try_write(&path, "struct Treasure;").unwrap();

        let text = adapter.try_read(&path).unwrap();
        remove_dir_all(&directory).unwrap();
        assert_eq!(text, "struct Treasure;");
    }

    #[test]
    fn a_file_the_adapter_takes_twice_has_what_came_last() {
        let adapter = FileSystem;
        let directory = std::env::temp_dir().join("language-rewritten");
        remove_dir_all(&directory).ok();
        create_dir_all(&directory).unwrap();
        let path = directory.join("mod.rs");
        adapter.try_write(&path, "struct Treasure;").unwrap();

        adapter.try_write(&path, "struct Chest;").unwrap();

        let text = adapter.try_read(&path).unwrap();
        remove_dir_all(&directory).unwrap();
        assert_eq!(text, "struct Chest;");
    }

    #[test]
    fn the_path_the_adapter_takes_first_is_beside_the_path_it_replaces() {
        let path = Path::new("crates/bytes/docs/UBIQUITOUS_LANGUAGE.md");

        let beside = FileSystem::beside(path);

        assert_eq!(beside.parent(), path.parent());
    }

    #[test]
    fn a_file_keeps_what_it_had_where_the_writing_failed() {
        let adapter = FileSystem;
        let directory = std::env::temp_dir().join("language-unwritten");
        remove_dir_all(&directory).ok();
        create_dir_all(&directory).unwrap();
        let path = directory.join("mod.rs");
        adapter.try_write(&path, "struct Treasure;").unwrap();
        create_dir_all(FileSystem::beside(&path)).unwrap();

        let response = adapter.try_write(&path, "struct Chest;");

        let text = adapter.try_read(&path).unwrap();
        remove_dir_all(&directory).unwrap();
        assert!(response.is_err());
        assert_eq!(text, "struct Treasure;");
    }

    #[test]
    fn nothing_is_left_beside_a_path_the_writing_could_not_take() {
        let adapter = FileSystem;
        let directory = std::env::temp_dir().join("language-unnamed");
        remove_dir_all(&directory).ok();
        create_dir_all(&directory).unwrap();
        let path = directory.join("mod.rs");
        create_dir_all(&path).unwrap();

        let response = adapter.try_write(&path, "struct Chest;");

        let count = read_dir(&directory).unwrap().count();
        remove_dir_all(&directory).unwrap();
        assert!(response.is_err());
        assert_eq!(count, 1);
    }

    #[test]
    fn the_file_system_hands_back_the_error_of_the_host() {
        let adapter = FileSystem;
        let path = Path::new("nowhere/mod.rs");

        let text = adapter.try_read(path);

        let error = text.unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }
}
