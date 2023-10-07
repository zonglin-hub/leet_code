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

#[test]
fn test_valid_palindrome_680() {
    assert!(Solution::valid_palindrome_680(String::from("aba")));
    assert!(Solution::valid_palindrome_680(String::from("abca")));
    assert_eq!(Solution::valid_palindrome_680(String::from("abc")), false);
}
