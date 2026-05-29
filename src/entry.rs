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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn transient_path(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("squirrelt_test_{}_{}", stamp, name))
    }

    fn create_entry(name: &str, contents: &[u8], executable: bool) -> Entry {
        let path = transient_path(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(contents).unwrap();

        if executable {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut permissions = file.metadata().unwrap().permissions();
                permissions.set_mode(0o755);
                fs::set_permissions(&path, permissions).unwrap();
            }
        }

        let metadata = fs::metadata(&path).unwrap();
        let file_type = metadata.file_type();
        Entry {
            name: path.file_name().unwrap().to_os_string(),
            path,
            file_type,
            metadata,
        }
    }

    #[test]
    fn color_for_rs_file_returns_green() {
        let entry = create_entry("example.rs", b"fn main() {}", false);
        assert_eq!(color_for_entry(&entry), "\x1b[32m");
        fs::remove_file(entry.path).unwrap();
    }

    #[test]
    fn color_for_unknown_extension_returns_empty() {
        let entry = create_entry("file.unknown", b"data", false);
        assert_eq!(color_for_entry(&entry), "");
        fs::remove_file(entry.path).unwrap();
    }

    #[test]
    fn directory_color_is_blue() {
        let path = transient_path("dir_example");
        fs::create_dir(&path).unwrap();
        let metadata = fs::metadata(&path).unwrap();
        let entry = Entry {
            name: path.file_name().unwrap().to_os_string(),
            path: path.clone(),
            file_type: metadata.file_type(),
            metadata,
        };

        assert_eq!(color_for_entry(&entry), "\x1b[34m");
        fs::remove_dir(&path).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn executable_file_is_detected_as_green() {
        let entry = create_entry("script", b"#!/bin/sh\necho hi", true);
        assert_eq!(color_for_entry(&entry), "\x1b[32m");
        fs::remove_file(entry.path).unwrap();
    }

    #[cfg(not(unix))]
    #[test]
    fn executable_file_is_not_detected_on_non_unix() {
        let entry = create_entry("script", b"#!/bin/sh\necho hi", true);
        assert_eq!(color_for_entry(&entry), "");
        fs::remove_file(entry.path).unwrap();
    }
}
