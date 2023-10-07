use leet_code::leet_code::Solution;

#[test]
fn test_add_strings_415_v1() {
    assert_eq!(
        Solution::add_strings_415_v1("11".to_string(), "123".to_string()),
        "134".to_string()
    );
    assert_eq!(
        Solution::add_strings_415_v1("456".to_string(), "77".to_string()),
        "533".to_string()
    );
    assert_eq!(
        Solution::add_strings_415_v1("0".to_string(), "0".to_string()),
        "0".to_string()
    );
}
