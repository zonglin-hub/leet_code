use leet_code::leet_code::Solution;

#[test]
fn test_valid_palindrome() {
    assert!(Solution::valid_palindrome(String::from("aba")));
    assert!(Solution::valid_palindrome(String::from("abca")));
    assert_eq!(Solution::valid_palindrome(String::from("abc")), false);
}
