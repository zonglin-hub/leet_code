use leet_code::leet_code::Solution;

#[test]
fn test_convert_time() {
    assert_eq!(
        Solution::convert_time("02:30".to_string(), "04:35".to_string()),
        3
    );
    assert_eq!(
        Solution::convert_time("11:00".to_string(), "11:01".to_string()),
        1
    );
}
