use leet_code::leet_code::Solution;

#[test]
fn test_is_power_of_three() {
    assert!(Solution::is_power_of_three(27));
    assert_eq!(Solution::is_power_of_three(0), false);
    assert!(Solution::is_power_of_three(9));
    assert_eq!(Solution::is_power_of_three(45), false);
}
