use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_touch_create_file() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("testfile.txt");

    let mut cmd = Command::cargo_bin("touch-for-windows").unwrap();

    cmd.arg(file_path.to_str().unwrap()).assert().success();

    assert!(file_path.exists());
}
