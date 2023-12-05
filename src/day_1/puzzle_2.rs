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
        find_first_digit(input),
        find_last_digit(input)
    );

    match digits {
        (Some(first_digit), Some(last_digit)) => Ok(format!("{}{}", first_digit, last_digit).parse::<usize>()?),
        _ => bail!("No digits found in input `{}`", input)
    }
}

fn parse_nth_char(input: &str, n: usize) -> Option<usize> {
    input.chars().nth(n).map(|c| c.to_string().parse::<usize>().ok()).flatten()
}

static SPELLED_OUT_DIGITS: [(&str, usize); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9)
];

fn find_first_digit_with_index(input: &str) -> Option<(usize, usize)> {
    let first_digit_index = input.find(|c: char| c.is_digit(10));

    match first_digit_index {
        Some(index) => parse_nth_char(input, index).map(|first_digit| (first_digit, index)),
        None => None
    }
}

fn find_first_spelled_digit_with_index(input: &str) -> Option<(usize, usize)> {
    SPELLED_OUT_DIGITS.iter()
        .filter_map(|(digit_spelling, digit)| {
            let index = input.find(digit_spelling);
            match index {
                Some(index) => Some((*digit, index)),
                None => None
            }
        })
        .min_by(|(_, x), (_, y)| x.cmp(y))
}

fn find_first_digit(input: &str) -> Option<usize> {
    vec!(
        find_first_digit_with_index(input),
        find_first_spelled_digit_with_index(input)
    ).iter()
    .filter_map(|digit_with_index| *digit_with_index)
    .min_by(|(_, x), (_, y)| x.cmp(y))
    .map(|(digit, _)| digit)
}

fn find_last_digit_with_index(input: &str) -> Option<(usize, usize)> {
    let last_digit_index = input.rfind(|c: char| c.is_digit(10));

    match last_digit_index {
        Some(index) => parse_nth_char(input, index).map(|last_digit| (last_digit, index)),
        None => None
    }
}

fn find_last_spelled_digit_with_index(input: &str) -> Option<(usize, usize)> {
    SPELLED_OUT_DIGITS.iter()
        .filter_map(|(digit_spelling, digit)| {
            let index = input.rfind(digit_spelling);
            match index {
                Some(index) => Some((*digit, index)),
                None => None
            }
        })
        .max_by(|(_, x), (_, y)| x.cmp(y))
}

fn find_last_digit(input: &str) -> Option<usize> {
    vec!(
        find_last_digit_with_index(input),
        find_last_spelled_digit_with_index(input)
    ).iter()
    .filter_map(|digit_with_index| *digit_with_index)
    .max_by(|(_, x), (_, y)| x.cmp(y))
    .map(|(digit, _)| digit)
}
