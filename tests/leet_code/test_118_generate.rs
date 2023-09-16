use leet_code::leet_code::Solution;

#[test]
fn test() {
    assert_eq!(Solution::generate(1), vec![vec![1]]);
    assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
}
