use std::{
    fs::{self},
    path::PathBuf,
    time::SystemTime,
};

use assert_cmd::Command;
use chrono::Local;
use rand::Rng;
use std::time::Duration;
use tempfile::{tempdir, TempDir};

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

    let test_file_name: String = format!("testfile-{}", rand::thread_rng().gen_range(0..=100));
    let file_path: PathBuf = temp_dir.path().join(&test_file_name);

    // File::create(&file_path).unwrap();

    let mut cmd: Command = Command::cargo_bin("touch").unwrap();

    cmd.arg("-a")
        .arg(file_path.to_str().unwrap())
        .assert()
        .success();

    let metadata: fs::Metadata = fs::metadata(&file_path).unwrap();
    let atime: SystemTime = metadata.accessed().unwrap();
    let now: SystemTime = SystemTime::now();

    assert!(
        now.duration_since(atime)
            // unnecessary
            .unwrap_or_else(|_| { Duration::new(0, 0) })
            // unnecessary ends
            .as_secs()
            < 2
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

    assert!(file_path.exists());

    let metadata: fs::Metadata = fs::metadata(&file_path).unwrap();
    let mtime: SystemTime = metadata.modified().unwrap();

    let expected_time: chrono::DateTime<chrono::Utc> =
        dateparser::parse_with_timezone(&date, &Local).unwrap();

    // Extract the seconds attribute for comparison, because I don't know
    // why the interval attribute of SystemTime is different for modified time
    // and expected time even if they're both parsed the same way
    // Oh, I think I know.
    // Since the time attribute is not specified for the date above, the current
    // time used during parsing is different for modified time and expected time.
    // I need to find a way to ignore the current time when it's not specified and
    // stick to using 00:00:00 (HH:MM:SS) as the default time during parsing.
    assert_eq!(
        filetime::FileTime::from_system_time(mtime).seconds(),
        filetime::FileTime::from_system_time(expected_time.into()).seconds()
    );
}
