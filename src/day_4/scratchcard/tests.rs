use super::Scratchcard;

#[test]
fn parse() {
    let result1 = Scratchcard::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    assert_eq!(result1.unwrap(), Scratchcard::new(1, vec!(41, 48, 83, 86, 17), vec!(83, 86, 6, 31, 17, 9, 48, 53)));
    let result2 = Scratchcard::parse("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
    assert_eq!(result2.unwrap(), Scratchcard::new(2, vec!(13, 32, 20, 16, 61), vec!(61, 30, 68, 82, 17, 32, 24, 19)));
    let result3 = Scratchcard::parse("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
    assert_eq!(result3.unwrap(), Scratchcard::new(3, vec!(1, 21, 53, 59, 44), vec!(69, 82, 63, 72, 16, 21, 14, 1)));
    let result4 = Scratchcard::parse("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
    assert_eq!(result4.unwrap(), Scratchcard::new(4, vec!(41, 92, 73, 84, 69), vec!(59, 84, 76, 51, 58, 5, 54, 83)));
    let result5 = Scratchcard::parse("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
    assert_eq!(result5.unwrap(), Scratchcard::new(5, vec!(87, 83, 26, 28, 32), vec!(88, 30, 70, 12, 93, 22, 82, 36)));
    let result6 = Scratchcard::parse("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
    assert_eq!(result6.unwrap(), Scratchcard::new(6, vec!(31, 18, 13, 56, 72), vec!(74, 77, 10, 23, 35, 67, 36, 11)));
}

#[test]
fn parse_id() {
    let result1 = Scratchcard::parse_id("Card   1");
    assert_eq!(result1.unwrap(), 1);
    let result2 = Scratchcard::parse_id("Card 200");
    assert_eq!(result2.unwrap(), 200);
}

#[test]
fn parse_numbers() {
    let result1 = Scratchcard::parse_numbers(" 41 48 83 86 17 ");
    assert_eq!(result1.unwrap(), vec!(41, 48, 83, 86, 17));
    let result2 = Scratchcard::parse_numbers(" 61 30 68 82 17 32 24 19");
    assert_eq!(result2.unwrap(), vec!(61, 30, 68, 82, 17, 32, 24, 19));
    let result3 = Scratchcard::parse_numbers("  1 21 53 59 44 ");
    assert_eq!(result3.unwrap(), vec!(1, 21, 53, 59, 44));
    let result4 = Scratchcard::parse_numbers(" 59 84 76 51 58  5 54 83");
    assert_eq!(result4.unwrap(), vec!(59, 84, 76, 51, 58, 5, 54, 83));
    let result5 = Scratchcard::parse_numbers(" 87 83 26 28 32 ");
    assert_eq!(result5.unwrap(), vec!(87, 83, 26, 28, 32));
    let result6 = Scratchcard::parse_numbers(" 74 77 10 23 35 67 36 11");
    assert_eq!(result6.unwrap(), vec!(74, 77, 10, 23, 35, 67, 36, 11));
}

#[test]
fn get_matching_numbers() {
    let scratchcard1 = Scratchcard::new(
        1,
        vec!(41, 48, 83, 86, 17),
        vec!(83, 86, 6, 31, 17, 9, 48, 53)
    );
    assert_eq!(scratchcard1.get_matching_numbers(), vec!(83, 86, 17, 48));
    let scratchcard2 = Scratchcard::new(
        2,
        vec!(13, 32, 20, 16, 61),
        vec!(61, 30, 68, 82, 17, 32, 24, 19)
    );
    assert_eq!(scratchcard2.get_matching_numbers(), vec!(61, 32));
    let scratchcard3 = Scratchcard::new(
        3,
        vec!(1, 21, 53, 59, 44),
        vec!(69, 82, 63, 72, 16, 21, 14, 1)
    );
    assert_eq!(scratchcard3.get_matching_numbers(), vec!(21, 1));
    let scratchcard4 = Scratchcard::new(
        4,
        vec!(41, 92, 73, 84, 69),
        vec!(59, 84, 76, 51, 58, 5, 54, 83)
    );
    assert_eq!(scratchcard4.get_matching_numbers(), vec!(84));
    let scratchcard5 = Scratchcard::new(
        5,
        vec!(87, 83, 26, 28, 32),
        vec!(88, 30, 70, 12, 93, 22, 82, 36)
    );
    assert_eq!(scratchcard5.get_matching_numbers(), vec!());
    let scratchcard6 = Scratchcard::new(
        6,
        vec!(31, 18, 13, 56, 72),
        vec!(74, 77, 10, 23, 35, 67, 36, 11)
    );
    assert_eq!(scratchcard6.get_matching_numbers(), vec!());
}

#[test]
fn get_points() {
    let scratchcard1 = Scratchcard::new(
        1,
        vec!(41, 48, 83, 86, 17),
        vec!(83, 86, 6, 31, 17, 9, 48, 53)
    );
    assert_eq!(scratchcard1.get_points().unwrap(), 8);
    let scratchcard2 = Scratchcard::new(
        2,
        vec!(13, 32, 20, 16, 61),
        vec!(61, 30, 68, 82, 17, 32, 24, 19)
    );
    assert_eq!(scratchcard2.get_points().unwrap(), 2);
    let scratchcard3 = Scratchcard::new(
        3,
        vec!(1, 21, 53, 59, 44),
        vec!(69, 82, 63, 72, 16, 21, 14, 1)
    );
    assert_eq!(scratchcard3.get_points().unwrap(), 2);
    let scratchcard4 = Scratchcard::new(
        4,
        vec!(41, 92, 73, 84, 69),
        vec!(59, 84, 76, 51, 58, 5, 54, 83)
    );
    assert_eq!(scratchcard4.get_points().unwrap(), 1);
    let scratchcard5 = Scratchcard::new(
        5,
        vec!(87, 83, 26, 28, 32),
        vec!(88, 30, 70, 12, 93, 22, 82, 36)
    );
    assert_eq!(scratchcard5.get_points().unwrap(), 0);
    let scratchcard6 = Scratchcard::new(
        6,
        vec!(31, 18, 13, 56, 72),
        vec!(74, 77, 10, 23, 35, 67, 36, 11)
    );
    assert_eq!(scratchcard6.get_points().unwrap(), 0);
}
