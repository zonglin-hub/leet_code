use leet_code::leet_code::Solution;

#[test]
fn test_num_of_strings() {
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

#[test]
fn test_num_of_strings_v1() {
    assert_eq!(
        Solution::num_of_strings_v1(
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
        Solution::num_of_strings_v1(
            vec!["a".to_string(), "b".to_string(), "c".to_string(),],
            "aaaaabbbbb".to_string()
        ),
        2
    );

    assert_eq!(
        Solution::num_of_strings_v1(
            vec!["a".to_string(), "a".to_string(), "a".to_string(),],
            "ab".to_string()
        ),
        3
    );
}
