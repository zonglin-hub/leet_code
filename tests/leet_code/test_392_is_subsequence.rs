use leet_code::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(
        Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
        true
    );
    assert_eq!(
        Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
        false
    );
    assert_eq!(
        Solution::is_subsequence("acb".to_string(), "ahbgdc".to_string()),
        false
    );
}
