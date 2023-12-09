use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Create temp files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn solve_puzzle_1() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("input.txt")?;
    input_file.write_str(
        vec!(
            "467..114..\n",
            "...*......\n",
            "..35..633.\n",
            "......#...\n",
            "617*......\n",
            ".....+.58.\n",
            "..592.....\n",
            "......755.\n",
            "...$.*....\n",
            ".664.598.."
        )
        .join("")
        .as_str()
    )?;

    let mut cmd = Command::cargo_bin("aoc")?;
    cmd.args([
        "--day=3",
        "--puzzle=1",
        "--input",
        input_file.path().to_str().unwrap_or_default(),
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("4361"));

    Ok(())
}

#[test]
fn solve_puzzle_2() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("input.txt")?;
    input_file.write_str(
        vec!(
            "467..114..\n",
            "...*......\n",
            "..35..633.\n",
            "......#...\n",
            "617*......\n",
            ".....+.58.\n",
            "..592.....\n",
            "......755.\n",
            "...$.*....\n",
            ".664.598.."
        )
        .join("")
        .as_str()
    )?;

    let mut cmd = Command::cargo_bin("aoc")?;
    cmd.args([
        "--day=3",
        "--puzzle=2",
        "--input",
        input_file.path().to_str().unwrap_or_default(),
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("467835"));

    Ok(())
}
