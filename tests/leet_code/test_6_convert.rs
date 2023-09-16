use leet_code::leet_code::Solution;

#[test]
fn test_convert_v1() {
    assert_eq!(
        Solution::convert_v1(String::from("PAYPALISHIRING"), 3),
        String::from("PAHNAPLSIIGYIR")
    );
    assert_eq!(
        Solution::convert_v1(String::from("PAYPALISHIRING"), 4),
        String::from("PINALSIGYAHRPI")
    );
    assert_eq!(
        Solution::convert_v1(String::from("A"), 1),
        String::from("A")
    );
}
