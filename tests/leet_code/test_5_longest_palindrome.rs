use leet_code::leet_code::Solution;

#[test]
fn test_longest_palindrome_5_v1() {
    assert_eq!(
        Solution::longest_palindrome_5_v1("babad".to_owned()),
        "bab".to_owned()
    );
    assert_eq!(
        Solution::longest_palindrome_5_v1("cbbd".to_owned()),
        "bb".to_owned()
    );
}
