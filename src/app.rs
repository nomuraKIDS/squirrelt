use std::fs;
use std::io;
use std::path::Path;

use crate::config::Config;
use crate::entry::{color_for_entry, Entry};

pub fn run(config: Config) -> io::Result<()> {
    let mut entries = read_entries(&config.path)?;

    if !config.show_all {
        entries.retain(|entry| {
            if let Some(name) = entry.name.to_str() {
                !name.starts_with('.')
            } else {
                true
            }
        });
    }

    sort_entries(&mut entries, &config);

    for entry in entries {
        let name = entry.name.to_string_lossy();
        let color = color_for_entry(&entry);
        let reset = "\x1b[0m";

        if color.is_empty() {
            println!("{}", name);
        } else {
            print!("{}", color);
            println!("{}{}", name, reset);
        }
    }

    Ok(())
}

fn read_entries(path: &Path) -> io::Result<Vec<Entry>> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)?;
        let file_type = metadata.file_type();

        entries.push(Entry {
            name: entry.file_name(),
            path,
            file_type,
            metadata,
        });
    }

    Ok(entries)
}

fn sort_entries(entries: &mut [Entry], config: &Config) {
    if config.sort_time {
        entries.sort_by(|a, b| {
            let a_time = a.metadata.modified().ok();
            let b_time = b.metadata.modified().ok();
            b_time.cmp(&a_time)
        });
    } else if config.sort_size {
        entries.sort_by_key(|entry| std::cmp::Reverse(entry.metadata.len()));
    } else {
        entries.sort_by(|a, b| a.name.cmp(&b.name));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use std::thread::sleep;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn transient_dir(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("squirrelt_test_dir_{}_{}", stamp, name));
        fs::create_dir(&dir).unwrap();
        dir
    }

    fn create_file(path: &PathBuf, contents: &[u8]) {
        let mut file = File::create(path).unwrap();
        file.write_all(contents).unwrap();
    }

    #[test]
    fn read_entries_returns_expected_files() {
        let dir = transient_dir("read_entries");
        let file_a = dir.join("a.txt");
        let file_b = dir.join("b.txt");

        create_file(&file_a, b"a");
        create_file(&file_b, b"b");

        let entries = read_entries(&dir).unwrap();
        let names: Vec<_> = entries
            .iter()
            .map(|entry| entry.name.to_string_lossy().into_owned())
            .collect();

        assert!(names.contains(&"a.txt".to_string()));
        assert!(names.contains(&"b.txt".to_string()));

        fs::remove_file(file_a).unwrap();
        fs::remove_file(file_b).unwrap();
        fs::remove_dir(dir).unwrap();
    }

    #[test]
    fn sort_entries_defaults_to_alphabetical() {
        let dir = transient_dir("alphabetical");
        let file_b = dir.join("b.txt");
        let file_a = dir.join("a.txt");

        create_file(&file_b, b"b");
        create_file(&file_a, b"a");

        let mut entries = read_entries(&dir).unwrap();
        let config = Config {
            show_all: false,
            sort_size: false,
            sort_time: false,
            path: dir.clone(),
        };

        sort_entries(&mut entries, &config);
        let names: Vec<_> = entries
            .iter()
            .map(|entry| entry.name.to_string_lossy().into_owned())
            .collect();

        assert_eq!(names, vec!["a.txt".to_string(), "b.txt".to_string()]);

        fs::remove_file(file_a).unwrap();
        fs::remove_file(file_b).unwrap();
        fs::remove_dir(dir).unwrap();
    }

    #[test]
    fn sort_entries_by_size_descending() {
        let dir = transient_dir("size_sort");
        let small = dir.join("small");
        let large = dir.join("large");

        create_file(&small, b"x");
        create_file(&large, b"xxxxxxxxxx");

        let mut entries = read_entries(&dir).unwrap();
        let config = Config {
            show_all: false,
            sort_size: true,
            sort_time: false,
            path: dir.clone(),
        };

        sort_entries(&mut entries, &config);
        let names: Vec<_> = entries
            .iter()
            .map(|entry| entry.name.to_string_lossy().into_owned())
            .collect();

        assert_eq!(names, vec!["large".to_string(), "small".to_string()]);

        fs::remove_file(small).unwrap();
        fs::remove_file(large).unwrap();
        fs::remove_dir(dir).unwrap();
    }

    #[test]
    fn sort_entries_by_modified_time_descending() {
        let dir = transient_dir("time_sort");
        let first = dir.join("first");
        let second = dir.join("second");

        create_file(&first, b"first");
        sleep(Duration::from_millis(10));
        create_file(&second, b"second");

        let mut entries = read_entries(&dir).unwrap();
        let config = Config {
            show_all: false,
            sort_size: false,
            sort_time: true,
            path: dir.clone(),
        };

        sort_entries(&mut entries, &config);
        let names: Vec<_> = entries
            .iter()
            .map(|entry| entry.name.to_string_lossy().into_owned())
            .collect();

        assert_eq!(names, vec!["second".to_string(), "first".to_string()]);

        fs::remove_file(first).unwrap();
        fs::remove_file(second).unwrap();
        fs::remove_dir(dir).unwrap();
    }
}
