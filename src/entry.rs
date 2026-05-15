use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

pub struct Entry {
    pub name: OsString,
    pub path: PathBuf,
    pub file_type: fs::FileType,
    pub metadata: fs::Metadata,
}

pub fn color_for_entry(entry: &Entry) -> &'static str {
    if entry.file_type.is_dir() {
        "\x1b[34m"
    } else if entry.file_type.is_symlink() {
        "\x1b[36m"
    } else if is_executable(entry) {
        "\x1b[32m"
    } else {
        ""
    }
}

pub fn is_executable(entry: &Entry) -> bool {
    if entry.file_type.is_file() {
        is_executable_metadata(&entry.metadata)
    } else {
        false
    }
}

#[cfg(unix)]
fn is_executable_metadata(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable_metadata(_metadata: &fs::Metadata) -> bool {
    false
}
