use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fc")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn test_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "THIS IS LINE 1\nTHIS IS LINE 2\nTHIS IS LINE 3\nTHIS IS LINE 4")?;
    
    let mut cmd = Command::cargo_bin("fc")?;
    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("THIS IS LINE 1"));

    Ok(())
}

#[test]
fn test_file_with_head_tail() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "THIS IS LINE 1\nTHIS IS LINE 2\nTHIS IS LINE 3")?;
    
    let mut cmd = Command::cargo_bin("fc")?;
    cmd.arg(file.path()).arg("--head=2").arg("--tail=2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("THIS IS LINE 2").count(2));

    Ok(())
}