use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Create temp files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn solve_puzzle_1() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("input.txt")?;
    input_file.write_str(
        vec!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        )
        .join("")
        .as_str()
    )?;

    let mut cmd = Command::cargo_bin("aoc")?;
    cmd.args([
        "--day=2",
        "--puzzle=1",
        "--input",
        input_file.path().to_str().unwrap_or_default(),
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("8"));

    Ok(())
}
