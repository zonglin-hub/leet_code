use leet_code::Solution;

#[test]
fn test_max_profit_v1() {
    assert_eq!(Solution::max_profit_v1(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit_v1(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_max_profit_v2() {
    assert_eq!(Solution::max_profit_v2(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit_v2(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_max_profit_v3() {
    assert_eq!(Solution::max_profit_v3(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit_v3(vec![7, 6, 4, 3, 1]), 0);
}
