use leet_code::Solution;

#[test]
fn test_score_of_parentheses_v1() {
    assert_eq!(Solution::score_of_parentheses_v1("()".to_string()), 1);
    assert_eq!(Solution::score_of_parentheses_v1("(())".to_string()), 2);
    assert_eq!(Solution::score_of_parentheses_v1("()()".to_string()), 2);
    assert_eq!(Solution::score_of_parentheses_v1("(()(()))".to_string()), 6);
}
