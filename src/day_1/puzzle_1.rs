#[cfg(test)]
mod tests;

use anyhow::{bail, Result};

pub fn solve(input: std::str::Lines<'_>) -> Result<usize> {
    let values = input
        .map(|input_value| recover_calibration_value(input_value))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<usize>();

    Ok(values)
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
