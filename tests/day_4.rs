use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Create temp files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn solve_puzzle_1() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("input.txt")?;
    input_file.write_str(
        vec!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        )
        .join("")
        .as_str()
    )?;

    let mut cmd = Command::cargo_bin("aoc")?;
    cmd.args([
        "--day=4",
        "--puzzle=1",
        "--input",
        input_file.path().to_str().unwrap_or_default(),
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("13"));

    Ok(())
}
