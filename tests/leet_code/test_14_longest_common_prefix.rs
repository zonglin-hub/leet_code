use leet_code::Solution;

#[test]
fn test_longest_common_prefix_v1() {
    assert_eq!(
        Solution::longest_common_prefix_v1(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl"
    );
    assert_eq!(
        Solution::longest_common_prefix_v1(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        ""
    );
}
