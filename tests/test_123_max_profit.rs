use leet_code::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::max_profit(vec![1]), 0);
}
