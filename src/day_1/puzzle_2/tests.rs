#[test]
fn solve_puzzle() {
    let result = super::solve("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".lines());
    assert_eq!(result.unwrap(), 281);
}

#[test]
fn recover_calibration_values() {
    let result1 = super::recover_calibration_value("two1nine");
    assert_eq!(result1.unwrap(), 29);
    let result2 = super::recover_calibration_value("eightwothree");
    assert_eq!(result2.unwrap(), 83);
    let result3 = super::recover_calibration_value("abcone2threexyz");
    assert_eq!(result3.unwrap(), 13);
    let result4 = super::recover_calibration_value("xtwone3four");
    assert_eq!(result4.unwrap(), 24);
    let result4 = super::recover_calibration_value("4nineeightseven2");
    assert_eq!(result4.unwrap(), 42);
    let result4 = super::recover_calibration_value("zoneight234");
    assert_eq!(result4.unwrap(), 14);
    let result4 = super::recover_calibration_value("7pqrstsixteen");
    assert_eq!(result4.unwrap(), 76);
}

#[test]
fn find_first_digits_with_index() {
    let result1 = super::find_first_digit_with_index("two1nine");
    assert_eq!(result1, Some((1, 3)));
    let result2 = super::find_first_digit_with_index("eightwothree");
    assert_eq!(result2, None);
    let result3 = super::find_first_digit_with_index("abcone2threexyz");
    assert_eq!(result3, Some((2, 6)));
    let result4 = super::find_first_digit_with_index("xtwone3four");
    assert_eq!(result4, Some((3, 6)));
    let result4 = super::find_first_digit_with_index("4nineeightseven2");
    assert_eq!(result4, Some((4, 0)));
    let result4 = super::find_first_digit_with_index("zoneight234");
    assert_eq!(result4, Some((2, 8)));
    let result4 = super::find_first_digit_with_index("7pqrstsixteen");
    assert_eq!(result4, Some((7, 0)));
}

#[test]
fn find_first_spelled_digits_with_index() {
    let result1 = super::find_first_spelled_digit_with_index("two1nine");
    assert_eq!(result1, Some((2, 0)));
    let result2 = super::find_first_spelled_digit_with_index("eightwothree");
    assert_eq!(result2, Some((8, 0)));
    let result3 = super::find_first_spelled_digit_with_index("abcone2threexyz");
    assert_eq!(result3, Some((1, 3)));
    let result4 = super::find_first_spelled_digit_with_index("xtwone3four");
    assert_eq!(result4, Some((2, 1)));
    let result4 = super::find_first_spelled_digit_with_index("4nineeightseven2");
    assert_eq!(result4, Some((9, 1)));
    let result4 = super::find_first_spelled_digit_with_index("zoneight234");
    assert_eq!(result4, Some((1, 1)));
    let result4 = super::find_first_spelled_digit_with_index("7pqrstsixteen");
    assert_eq!(result4, Some((6, 6)));
}

#[test]
fn find_first_digits() {
    let result1 = super::find_first_digit("two1nine");
    assert_eq!(result1, Some(2));
    let result2 = super::find_first_digit("eightwothree");
    assert_eq!(result2, Some(8));
    let result3 = super::find_first_digit("abcone2threexyz");
    assert_eq!(result3, Some(1));
    let result4 = super::find_first_digit("xtwone3four");
    assert_eq!(result4, Some(2));
    let result4 = super::find_first_digit("4nineeightseven2");
    assert_eq!(result4, Some(4));
    let result4 = super::find_first_digit("zoneight234");
    assert_eq!(result4, Some(1));
    let result4 = super::find_first_digit("7pqrstsixteen");
    assert_eq!(result4, Some(7));
}

#[test]
fn find_last_digits_with_index() {
    let result1 = super::find_last_digit_with_index("two1nine");
    assert_eq!(result1, Some((1, 3)));
    let result2 = super::find_last_digit_with_index("eightwothree");
    assert_eq!(result2, None);
    let result3 = super::find_last_digit_with_index("abcone2threexyz");
    assert_eq!(result3, Some((2, 6)));
    let result4 = super::find_last_digit_with_index("xtwone3four");
    assert_eq!(result4, Some((3, 6)));
    let result4 = super::find_last_digit_with_index("4nineeightseven2");
    assert_eq!(result4, Some((2, 15)));
    let result4 = super::find_last_digit_with_index("zoneight234");
    assert_eq!(result4, Some((4, 10)));
    let result4 = super::find_last_digit_with_index("7pqrstsixteen");
    assert_eq!(result4, Some((7, 0)));
}

#[test]
fn find_last_spelled_digits_with_index() {
    let result1 = super::find_last_spelled_digit_with_index("two1nine");
    assert_eq!(result1, Some((9, 4)));
    let result2 = super::find_last_spelled_digit_with_index("eightwothree");
    assert_eq!(result2, Some((3, 7)));
    let result3 = super::find_last_spelled_digit_with_index("abcone2threexyz");
    assert_eq!(result3, Some((3, 7)));
    let result4 = super::find_last_spelled_digit_with_index("xtwone3four");
    assert_eq!(result4, Some((4, 7)));
    let result4 = super::find_last_spelled_digit_with_index("4nineeightseven2");
    assert_eq!(result4, Some((7, 10)));
    let result4 = super::find_last_spelled_digit_with_index("zoneight234");
    assert_eq!(result4, Some((8, 3)));
    let result4 = super::find_last_spelled_digit_with_index("7pqrstsixteen");
    assert_eq!(result4, Some((6, 6)));
}

#[test]
fn find_last_digits() {
    let result1 = super::find_last_digit("two1nine");
    assert_eq!(result1, Some(9));
    let result2 = super::find_last_digit("eightwothree");
    assert_eq!(result2, Some(3));
    let result3 = super::find_last_digit("abcone2threexyz");
    assert_eq!(result3, Some(3));
    let result4 = super::find_last_digit("xtwone3four");
    assert_eq!(result4, Some(4));
    let result4 = super::find_last_digit("4nineeightseven2");
    assert_eq!(result4, Some(2));
    let result4 = super::find_last_digit("zoneight234");
    assert_eq!(result4, Some(4));
    let result4 = super::find_last_digit("7pqrstsixteen");
    assert_eq!(result4, Some(6));
}
