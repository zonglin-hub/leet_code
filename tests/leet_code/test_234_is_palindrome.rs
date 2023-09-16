use leet_code::leet_code::{create_list, Solution};

#[test]
fn test_is_palindrome_v1() {
    assert_eq!(Solution::is_palindrome_v1(create_list(vec![1, 2])), false);
}
