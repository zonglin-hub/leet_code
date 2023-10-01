use leet_code::leet_code::Solution;

#[test]
fn test_find_kth_bit() {
    assert_eq!(
        Solution::num_of_strings(
            vec![
                "a".to_string(),
                "abc".to_string(),
                "bc".to_string(),
                "d".to_string()
            ],
            "abc".to_string()
        ),
        3
    );

    assert_eq!(
        Solution::num_of_strings(
            vec!["a".to_string(), "b".to_string(), "c".to_string(),],
            "aaaaabbbbb".to_string()
        ),
        2
    );

    assert_eq!(
        Solution::num_of_strings(
            vec!["a".to_string(), "a".to_string(), "a".to_string(),],
            "ab".to_string()
        ),
        3
    );
}
