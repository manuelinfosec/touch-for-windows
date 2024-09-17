use std::{
    fs::{self, File},
    path::PathBuf,
    time::SystemTime,
};

use assert_cmd::Command;
use chrono::{DateTime, Local, TimeZone};
use std::time::Duration;
use tempfile::tempdir;
use tempfile::TempDir;

#[test]
fn test_touch_create_file() {
    let temp_dir: tempfile::TempDir = tempdir().unwrap();
    let file_path: std::path::PathBuf = temp_dir.path().join("testfile.txt");

    let mut cmd: Command = Command::cargo_bin("touch").unwrap();

    cmd.arg(file_path.to_str().unwrap()).assert().success();

    assert!(file_path.exists());
}

#[test]
fn test_touch_modify_atime() {
    let temp_dir: TempDir = tempdir().unwrap();
    let file_path: PathBuf = temp_dir.path().join("testfile.txt");
    File::create(&file_path).unwrap();

    let mut cmd: Command = Command::cargo_bin("touch").unwrap();

    cmd.arg("-a")
        .arg(file_path.to_str().unwrap())
        .assert()
        .success();

    let metadata: fs::Metadata = fs::metadata(&file_path).unwrap();
    let atime: SystemTime = metadata.accessed().unwrap();
    let now: SystemTime = SystemTime::now();

    assert!(
        atime
            .duration_since(now)
            .unwrap_or_else(|_| Duration::new(0, 0))
            .as_secs()
            < 2 as u64
    );
}

#[test]
fn test_touch_create_file_with_date() {
    let temp_dir: TempDir = tempdir().unwrap();
    let file_path: PathBuf = temp_dir.path().join("testfile.txt");
    let date: &str = "2024-07-04";

    let mut cmd = Command::cargo_bin("touch").unwrap();

    cmd.arg("-d")
        .arg(date)
        .arg(file_path.to_str().unwrap())
        .assert()
        .success();
    println!("Success");

    assert!(file_path.exists());

    let metadata = fs::metadata(&file_path).unwrap();
    let mtime = metadata.modified().unwrap();
    println!("About parsing");
    let expected_time = chrono::Local
        .from_local_datetime(&chrono::NaiveDateTime::parse_from_str(date, "%Y-%m-%d").unwrap())
        .unwrap();
    println!("Finished parsing");

    assert_eq!(mtime, expected_time.into());
}
