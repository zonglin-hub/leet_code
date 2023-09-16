use leet_code::leet_code::Solution;

#[test]
fn test_is_match_v1() {
    assert_eq!(
        Solution::is_match_v1("aa".to_string(), "a".to_string()),
        false
    );
    assert_eq!(
        Solution::is_match_v1("aa".to_string(), "a*".to_string()),
        true
    );
    assert_eq!(
        Solution::is_match_v1("ab".to_string(), ".*".to_string()),
        true
    );
}
