use std::{
    fs::{self, File},
    path::PathBuf,
    time::SystemTime,
};

use assert_cmd::Command;
use rand::Rng;
use std::time::Duration;
use tempfile::{tempdir, TempDir};

#[test]
fn test_touch_create_file() {
    let temp_dir: tempfile::TempDir = tempdir().unwrap();
    
    let test_file_name: String = format!("testfile-{}", rand::thread_rng().gen_range(0..=100));
    println!("Test file name #1: {test_file_name}");
    let file_path: PathBuf = temp_dir.path().join(&test_file_name);

    let mut cmd: Command = Command::cargo_bin("touch").unwrap();

    cmd.arg(file_path.to_str().unwrap()).assert().success();

    assert!(file_path.exists());
}

#[test]
fn test_touch_modify_atime() {
    let temp_dir: TempDir = tempdir().unwrap();

    let test_file_name: String = format!("testfile-{}", rand::thread_rng().gen_range(0..=100));
    println!("Test file name #2: {test_file_name}");
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
        atime
            .duration_since(now)
            .unwrap_or_else(|_| Duration::new(0, 0))
            .as_secs()
            < 2
    );
}
