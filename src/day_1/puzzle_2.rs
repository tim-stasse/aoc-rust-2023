use anyhow::{bail, Result};

#[test]
fn solve_puzzle() {
    let result = solve("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".lines());
    assert_eq!(result.unwrap(), 281);
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
    let result1 = recover_calibration_value("two1nine");
    assert_eq!(result1.unwrap(), 29);
    let result2 = recover_calibration_value("eightwothree");
    assert_eq!(result2.unwrap(), 83);
    let result3 = recover_calibration_value("abcone2threexyz");
    assert_eq!(result3.unwrap(), 13);
    let result4 = recover_calibration_value("xtwone3four");
    assert_eq!(result4.unwrap(), 24);
    let result4 = recover_calibration_value("4nineeightseven2");
    assert_eq!(result4.unwrap(), 42);
    let result4 = recover_calibration_value("zoneight234");
    assert_eq!(result4.unwrap(), 14);
    let result4 = recover_calibration_value("7pqrstsixteen");
    assert_eq!(result4.unwrap(), 76);
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

#[test]
fn find_first_digits_with_index() {
    let result1 = find_first_digit_with_index("two1nine");
    assert_eq!(result1, Some((1, 3)));
    let result2 = find_first_digit_with_index("eightwothree");
    assert_eq!(result2, None);
    let result3 = find_first_digit_with_index("abcone2threexyz");
    assert_eq!(result3, Some((2, 6)));
    let result4 = find_first_digit_with_index("xtwone3four");
    assert_eq!(result4, Some((3, 6)));
    let result4 = find_first_digit_with_index("4nineeightseven2");
    assert_eq!(result4, Some((4, 0)));
    let result4 = find_first_digit_with_index("zoneight234");
    assert_eq!(result4, Some((2, 8)));
    let result4 = find_first_digit_with_index("7pqrstsixteen");
    assert_eq!(result4, Some((7, 0)));
}

fn find_first_digit_with_index(input: &str) -> Option<(usize, usize)> {
    let first_digit_index = input.find(|c: char| c.is_digit(10));

    match first_digit_index {
        Some(index) => parse_nth_char(input, index).map(|first_digit| (first_digit, index)),
        None => None
    }
}

#[test]
fn find_first_spelled_digits_with_index() {
    let result1 = find_first_spelled_digit_with_index("two1nine");
    assert_eq!(result1, Some((2, 0)));
    let result2 = find_first_spelled_digit_with_index("eightwothree");
    assert_eq!(result2, Some((8, 0)));
    let result3 = find_first_spelled_digit_with_index("abcone2threexyz");
    assert_eq!(result3, Some((1, 3)));
    let result4 = find_first_spelled_digit_with_index("xtwone3four");
    assert_eq!(result4, Some((2, 1)));
    let result4 = find_first_spelled_digit_with_index("4nineeightseven2");
    assert_eq!(result4, Some((9, 1)));
    let result4 = find_first_spelled_digit_with_index("zoneight234");
    assert_eq!(result4, Some((1, 1)));
    let result4 = find_first_spelled_digit_with_index("7pqrstsixteen");
    assert_eq!(result4, Some((6, 6)));
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

#[test]
fn find_first_digits() {
    let result1 = find_first_digit("two1nine");
    assert_eq!(result1, Some(2));
    let result2 = find_first_digit("eightwothree");
    assert_eq!(result2, Some(8));
    let result3 = find_first_digit("abcone2threexyz");
    assert_eq!(result3, Some(1));
    let result4 = find_first_digit("xtwone3four");
    assert_eq!(result4, Some(2));
    let result4 = find_first_digit("4nineeightseven2");
    assert_eq!(result4, Some(4));
    let result4 = find_first_digit("zoneight234");
    assert_eq!(result4, Some(1));
    let result4 = find_first_digit("7pqrstsixteen");
    assert_eq!(result4, Some(7));
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

#[test]
fn find_last_digits_with_index() {
    let result1 = find_last_digit_with_index("two1nine");
    assert_eq!(result1, Some((1, 3)));
    let result2 = find_last_digit_with_index("eightwothree");
    assert_eq!(result2, None);
    let result3 = find_last_digit_with_index("abcone2threexyz");
    assert_eq!(result3, Some((2, 6)));
    let result4 = find_last_digit_with_index("xtwone3four");
    assert_eq!(result4, Some((3, 6)));
    let result4 = find_last_digit_with_index("4nineeightseven2");
    assert_eq!(result4, Some((2, 15)));
    let result4 = find_last_digit_with_index("zoneight234");
    assert_eq!(result4, Some((4, 10)));
    let result4 = find_last_digit_with_index("7pqrstsixteen");
    assert_eq!(result4, Some((7, 0)));
}

fn find_last_digit_with_index(input: &str) -> Option<(usize, usize)> {
    let last_digit_index = input.rfind(|c: char| c.is_digit(10));

    match last_digit_index {
        Some(index) => parse_nth_char(input, index).map(|last_digit| (last_digit, index)),
        None => None
    }
}

#[test]
fn find_last_spelled_digits_with_index() {
    let result1 = find_last_spelled_digit_with_index("two1nine");
    assert_eq!(result1, Some((9, 4)));
    let result2 = find_last_spelled_digit_with_index("eightwothree");
    assert_eq!(result2, Some((3, 7)));
    let result3 = find_last_spelled_digit_with_index("abcone2threexyz");
    assert_eq!(result3, Some((3, 7)));
    let result4 = find_last_spelled_digit_with_index("xtwone3four");
    assert_eq!(result4, Some((4, 7)));
    let result4 = find_last_spelled_digit_with_index("4nineeightseven2");
    assert_eq!(result4, Some((7, 10)));
    let result4 = find_last_spelled_digit_with_index("zoneight234");
    assert_eq!(result4, Some((8, 3)));
    let result4 = find_last_spelled_digit_with_index("7pqrstsixteen");
    assert_eq!(result4, Some((6, 6)));
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

#[test]
fn find_last_digits() {
    let result1 = find_last_digit("two1nine");
    assert_eq!(result1, Some(9));
    let result2 = find_last_digit("eightwothree");
    assert_eq!(result2, Some(3));
    let result3 = find_last_digit("abcone2threexyz");
    assert_eq!(result3, Some(3));
    let result4 = find_last_digit("xtwone3four");
    assert_eq!(result4, Some(4));
    let result4 = find_last_digit("4nineeightseven2");
    assert_eq!(result4, Some(2));
    let result4 = find_last_digit("zoneight234");
    assert_eq!(result4, Some(4));
    let result4 = find_last_digit("7pqrstsixteen");
    assert_eq!(result4, Some(6));
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
