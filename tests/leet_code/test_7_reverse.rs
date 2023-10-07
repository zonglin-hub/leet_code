use leet_code::leet_code::Solution;

#[test]
fn test_reverse_7_v1() {
    assert_eq!(Solution::reverse_7_v1(123), 321);
    assert_eq!(Solution::reverse_7_v1(-123), -321);
    assert_eq!(Solution::reverse_7_v1(120), 21);
    assert_eq!(Solution::reverse_7_v1(0), 0);
}
