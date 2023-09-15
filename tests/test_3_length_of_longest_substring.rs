use leet_code::Solution;

#[test]
fn test_length_of_longest_substring_v1() {
    assert_eq!(
        Solution::length_of_longest_substring_v1("abcabcbb".to_string()),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring_v1("bbbbb".to_string()),
        1
    );
    assert_eq!(
        Solution::length_of_longest_substring_v1("pwwkew".to_string()),
        3
    );
}
