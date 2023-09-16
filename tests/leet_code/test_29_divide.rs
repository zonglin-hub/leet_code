use leet_code::leet_code::Solution;

#[test]
fn test_divide() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(7, -3), -2);
}
