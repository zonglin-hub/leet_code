use leet_code::Solution;

#[test]
fn test_is_valid_v1() {
    assert!(Solution::is_valid_v1(String::from("()")));
    assert!(Solution::is_valid_v1(String::from("()[]{}")));
    assert_eq!(Solution::is_valid_v1(String::from("(]")), false);
}
