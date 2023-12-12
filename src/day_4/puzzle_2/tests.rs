#[test]
fn solve() {
    let result = super::solve(
        vec!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        ).into_iter()
    );
    assert_eq!(result.unwrap(), 30);
}

#[test]
fn get_copy_range() {
    let result1 = super::get_copy_range(1, 4);
    assert_eq!(result1, 2..6);
    let result2 = super::get_copy_range(2, 2);
    assert_eq!(result2, 3..5);
    let result3 = super::get_copy_range(3, 2);
    assert_eq!(result3, 4..6);
}
