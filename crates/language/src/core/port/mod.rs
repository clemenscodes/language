use ddd::DrivenPort;
use std::io;
use std::path::Path;

pub(crate) trait Reader: DrivenPort {
    fn try_read(&self, path: &Path) -> io::Result<String>;
}

pub(crate) trait Writer: DrivenPort {
    fn try_write(&self, path: &Path, text: &str) -> io::Result<()>;

    fn has_directory(&self, path: &Path) -> bool;
}
