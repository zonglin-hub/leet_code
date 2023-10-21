use leet_code::leet_code::Solution;

#[test]
fn test_trap() {
    assert_eq!(
        Solution::multiply("2".to_string(), "3".to_string()),
        "6".to_string()
    );
    assert_eq!(
        Solution::multiply("123".to_string(), "456".to_string()),
        "56088".to_string()
    );
}
