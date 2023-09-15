use leet_code::Solution;

#[test]
fn test_add_strings() {
    assert_eq!(
        Solution::add_strings("11".to_string(), "123".to_string()),
        "134".to_string()
    );
    assert_eq!(
        Solution::add_strings("456".to_string(), "77".to_string()),
        "533".to_string()
    );
    assert_eq!(
        Solution::add_strings("0".to_string(), "0".to_string()),
        "0".to_string()
    );
}
