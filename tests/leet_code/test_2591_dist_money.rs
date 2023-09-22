use leet_code::leet_code::Solution;

#[test]
fn test_dist_money() {
    assert_eq!(Solution::dist_money(20, 3), 1);
    assert_eq!(Solution::dist_money(16, 2), 2);
    assert_eq!(Solution::dist_money(23, 2), 1);
}
