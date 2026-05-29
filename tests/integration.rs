use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn transient_dir(name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("squirrelt_integration_{}_{}", stamp, name));
    fs::create_dir(&dir).unwrap();
    dir
}

fn create_file(path: &PathBuf, contents: &[u8]) {
    let mut file = File::create(path).unwrap();
    file.write_all(contents).unwrap();
}

fn run_squirrelt(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_squirrelt"))
        .args(args)
        .output()
        .expect("failed to execute squirrelt")
}

#[test]
fn hidden_files_are_skipped_without_a_flag() {
    let dir = transient_dir("hidden_skip");
    create_file(&dir.join(".hidden"), b"secret");
    create_file(&dir.join("visible"), b"public");

    let output = run_squirrelt(&[dir.to_string_lossy().as_ref()]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("visible"));
    assert!(!stdout.contains(".hidden"));

    fs::remove_file(dir.join(".hidden")).unwrap();
    fs::remove_file(dir.join("visible")).unwrap();
    fs::remove_dir(dir).unwrap();
}

#[test]
fn hidden_files_are_shown_with_a_flag() {
    let dir = transient_dir("hidden_show");
    create_file(&dir.join(".hidden"), b"secret");
    create_file(&dir.join("visible"), b"public");

    let output = run_squirrelt(&["-a", dir.to_string_lossy().as_ref()]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("visible"));
    assert!(stdout.contains(".hidden"));

    fs::remove_file(dir.join(".hidden")).unwrap();
    fs::remove_file(dir.join("visible")).unwrap();
    fs::remove_dir(dir).unwrap();
}

#[test]
fn sort_size_option_outputs_larger_files_first() {
    let dir = transient_dir("sort_size");
    create_file(&dir.join("small"), b"x");
    create_file(&dir.join("large"), b"xxxxxxxxxx");

    let output = run_squirrelt(&["--sort-size", dir.to_string_lossy().as_ref()]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    let lines: Vec<_> = stdout.lines().collect();
    assert!(lines.iter().any(|line| line.contains("large")));
    assert!(lines.iter().any(|line| line.contains("small")));
    assert!(lines.iter().position(|line| line.contains("large"))
        < lines.iter().position(|line| line.contains("small")));

    fs::remove_file(dir.join("small")).unwrap();
    fs::remove_file(dir.join("large")).unwrap();
    fs::remove_dir(dir).unwrap();
}
