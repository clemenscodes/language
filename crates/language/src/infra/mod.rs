mod file_system;

use crate::core::Reader;
use crate::core::Writer;
use crate::infra::file_system::FileSystem;

pub(crate) fn reader() -> Box<dyn Reader> {
    let adapter = FileSystem;
    Box::new(adapter)
}

pub(crate) fn writer() -> Box<dyn Writer> {
    let adapter = FileSystem;
    Box::new(adapter)
}
