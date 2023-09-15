use leet_code::Solution;

#[test]
fn test_longest_valid_parentheses() {
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
}
