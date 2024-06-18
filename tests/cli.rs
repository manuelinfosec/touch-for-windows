use std::{
    alloc::System,
    env::temp_dir,
    fs::{self, File},
    path::PathBuf,
    time::SystemTime,
};

use assert_cmd::Command;
use chrono::Duration;
use tempfile::tempdir;

#[test]
fn test_touch_create_file() {
    let temp_dir: tempfile::TempDir = tempdir().unwrap();
    let file_path: std::path::PathBuf = temp_dir.path().join("testfile.txt");

    let mut cmd: Command = Command::cargo_bin("touch-for-windows").unwrap();

    cmd.arg(file_path.to_str().unwrap()).assert().success();

    assert!(file_path.exists());
}

#[test]
fn test_touch_modify_atime() {
    let temp_dir: TempDir = tempdir().unwrap();
    let file_path: PathBuf = temp_dir.path().join("testfile.txt");
    File::create(&file_path).unwrap();

    let mut cmd: Command = Command::cargo_bin("touch-for-windows").unwrap();

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
            .unwrap_or_else(|_| Duration::new(0, 0).as_secs())
            < 2
    );
}
