use leet_code::leet_code::Solution;

#[test]
fn test_is_palindrome_v2() {
    assert!(Solution::is_palindrome_v2(121));
    assert_eq!(Solution::is_palindrome_v2(-121), false);
    assert_eq!(Solution::is_palindrome_v2(10), false);
}
