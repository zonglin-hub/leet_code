use leet_code::Solution;

#[test]
fn test_restore_string() {
    assert_eq!(
        Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
        "leetcode".to_string()
    );
    assert_eq!(
        Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
        "abc".to_string()
    );
}
