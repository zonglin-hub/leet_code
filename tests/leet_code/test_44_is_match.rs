use leet_code::leet_code::Solution;

#[test]
fn test_is_match() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
    assert_eq!(
        Solution::is_match("cb".to_string(), "?a".to_string()),
        false
    );
}
