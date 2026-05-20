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
        "\x1b[34m" // blue for directories
    } else if entry.file_type.is_symlink() {
        "\x1b[36m" // cyan for symlinks
    } else if is_executable(entry) {
        "\x1b[32m" // green for executables
    } else if let Some(ext) = entry.path.extension().and_then(|e| e.to_str()) {
        color_by_extension(ext)
    } else {
        ""
    }
}

fn color_by_extension(extension: &str) -> &'static str {
    match extension.to_ascii_lowercase().as_str() {
        "rs" | "py" | "js" | "ts" | "java" | "go" | "c" | "cpp" | "h" | "sh" => "\x1b[32m",
        "md" | "txt" | "toml" | "json" | "yaml" | "yml" => "\x1b[33m",
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" => "\x1b[35m",
        "mp3" | "wav" | "ogg" | "flac" | "m4a" => "\x1b[36m",
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => "\x1b[31m",
        _ => "",
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
