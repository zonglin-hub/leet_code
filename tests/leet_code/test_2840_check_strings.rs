use leet_code::leet_code::Solution;

#[test]
fn test_longest_equal_subarray() {
    assert_eq!(
        Solution::check_strings("abcdba".to_string(), "cabdab".to_string()),
        true
    );
    assert_eq!(
        Solution::check_strings("abe".to_string(), "bea".to_string()),
        false
    );
}
