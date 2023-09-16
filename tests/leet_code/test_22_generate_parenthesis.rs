use leet_code::leet_code::Solution;

#[test]
fn test_generate_parenthesis_v1() {
    assert_eq!(
        Solution::generate_parenthesis_v1(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
    assert_eq!(Solution::generate_parenthesis_v1(1), vec!["()"])
}
