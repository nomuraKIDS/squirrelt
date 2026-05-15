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

fn sort_entries(entries: &mut Vec<Entry>, config: &Config) {
    if config.sort_time {
        entries.sort_by(|a, b| {
            let a_time = a.metadata.modified().ok();
            let b_time = b.metadata.modified().ok();
            b_time.cmp(&a_time)
        });
    } else if config.sort_size {
        entries.sort_by(|a, b| b.metadata.len().cmp(&a.metadata.len()));
    } else {
        entries.sort_by(|a, b| a.name.cmp(&b.name));
    }
}
