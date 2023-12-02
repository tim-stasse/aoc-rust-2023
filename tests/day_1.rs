use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Create temp files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn solve_puzzle_1() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("input.txt")?;
    input_file.write_str("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet")?;

    let mut cmd = Command::cargo_bin("aoc")?;
    cmd.args([
        "--day=1",
        "--puzzle=1",
        "--input",
        input_file.path().to_str().unwrap_or_default(),
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("142"));

    Ok(())
}

#[test]
fn solve_puzzle_2() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("input.txt")?;
    input_file.write_str("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen")?;

    let mut cmd = Command::cargo_bin("aoc")?;
    cmd.args([
        "--day=1",
        "--puzzle=2",
        "--input",
        input_file.path().to_str().unwrap_or_default(),
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("281"));

    Ok(())
}
