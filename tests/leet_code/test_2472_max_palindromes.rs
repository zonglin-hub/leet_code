use leet_code::leet_code::Solution;

#[test]
fn test_max_palindromes() {
    assert_eq!(Solution::max_palindromes(String::from("abaccdbbd"), 3), 2);
    assert_eq!(Solution::max_palindromes(String::from("adbcda"), 2), 0);
}
