use leet_code::leet_code::{create_list, Solution};

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
fn test_is_palindrome_9_v2() {
    assert!(Solution::is_palindrome_9_v2(121));
    assert_eq!(Solution::is_palindrome_9_v2(-121), false);
    assert_eq!(Solution::is_palindrome_9_v2(10), false);
}

#[test]
fn test_is_palindrome_v1() {
    assert_eq!(
        Solution::is_palindrome_234_v1(create_list(vec![1, 2])),
        false
    );
}

#[test]
fn test_max_palindromes() {
    assert_eq!(
        Solution::max_palindromes_2472(String::from("abaccdbbd"), 3),
        2
    );
    assert_eq!(Solution::max_palindromes_2472(String::from("adbcda"), 2), 0);
}

#[test]
fn test_valid_palindrome_680() {
    assert!(Solution::valid_palindrome_680(String::from("aba")));
    assert!(Solution::valid_palindrome_680(String::from("abca")));
    assert_eq!(Solution::valid_palindrome_680(String::from("abc")), false);
}
