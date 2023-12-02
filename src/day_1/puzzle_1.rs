use anyhow::{bail, Result};

#[test]
fn solve_puzzle() {
    let result = solve("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".lines());
    assert_eq!(result.unwrap(), 142);
}

pub fn solve(input: std::str::Lines<'_>) -> Result<usize> {
    let values = input
        .map(|input_value| recover_calibration_value(input_value))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<usize>();

    Ok(values)
}

#[test]
fn recover_calibration_values() {
    let result1 = recover_calibration_value("1abc2");
    assert_eq!(result1.unwrap(), 12);
    let result2 = recover_calibration_value("pqr3stu8vwx");
    assert_eq!(result2.unwrap(), 38);
    let result3 = recover_calibration_value("a1b2c3d4e5f");
    assert_eq!(result3.unwrap(), 15);
    let result4 = recover_calibration_value("treb7uchet");
    assert_eq!(result4.unwrap(), 77);
}

fn recover_calibration_value(input: &str) -> Result<usize> {
    let digits = (
        input.chars().find(|c| c.is_digit(10)),
        input.chars().rfind(|c| c.is_digit(10))
    );

    match digits {
        (Some(first_digit), Some(last_digit)) => Ok(format!("{}{}", first_digit, last_digit).parse::<usize>()?),
        _ => bail!("No digits found in input `{}`", input)
    }
}
