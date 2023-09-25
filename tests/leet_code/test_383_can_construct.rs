use leet_code::leet_code::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(
        Solution::is_subsequence("a".to_string(), "b".to_string()),
        false
    );
    assert_eq!(
        Solution::is_subsequence("aa".to_string(), "ab".to_string()),
        false
    );
    assert!(Solution::is_subsequence(
        "aa".to_string(),
        "aab".to_string()
    ));
}

#[test]
fn test_all() {
    let a = [1, 2, 3];

    assert!(a.iter().all(|&x| x > 0));

    assert!(!a.iter().all(|&x| x > 2));
}
