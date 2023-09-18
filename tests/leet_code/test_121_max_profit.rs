use leet_code::leet_code::Solution;

#[test]
fn test_max_profit_121_v1() {
    assert_eq!(Solution::max_profit_121_v1(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit_121_v1(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_max_profit_121_v2() {
    assert_eq!(Solution::max_profit_121_v2(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit_121_v2(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_max_profit_121_v3() {
    assert_eq!(Solution::max_profit_121_v3(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit_121_v3(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_max_profit_123() {
    assert_eq!(Solution::max_profit_123(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    assert_eq!(Solution::max_profit_123(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit_123(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::max_profit_123(vec![1]), 0);
}

#[test]
fn test_max_profit_122() {
    assert_eq!(Solution::max_profit_122(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(Solution::max_profit_122(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit_122(vec![7, 6, 4, 3, 1]), 0);
}
