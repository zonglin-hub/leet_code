use leet_code::Solution;

#[test]
fn test_is_power_of_four() {
    assert!(Solution::is_power_of_four(16));
    assert!(!Solution::is_power_of_four(5));
    assert!(Solution::is_power_of_four(1));
}
