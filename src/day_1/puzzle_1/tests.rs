#[test]
fn solve() {
    let result = super::solve("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".lines());
    assert_eq!(result.unwrap(), 142);
}

#[test]
fn recover_calibration_value() {
    let result1 = super::recover_calibration_value("1abc2");
    assert_eq!(result1.unwrap(), 12);
    let result2 = super::recover_calibration_value("pqr3stu8vwx");
    assert_eq!(result2.unwrap(), 38);
    let result3 = super::recover_calibration_value("a1b2c3d4e5f");
    assert_eq!(result3.unwrap(), 15);
    let result4 = super::recover_calibration_value("treb7uchet");
    assert_eq!(result4.unwrap(), 77);
}
