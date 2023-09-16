use leet_code::leet_code::Solution;

#[test]
fn test_str_str() {
    assert_eq!(
        Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
        0
    );
    assert_eq!(
        Solution::str_str("leetcode".to_string(), "leeto".to_string()),
        -1
    );
}
