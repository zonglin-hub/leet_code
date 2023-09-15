use leet_code::Solution;

#[test]
fn test_is_power_of_two_v1() {
    assert_eq!(Solution::is_power_of_two_v1(1), true);
    assert_eq!(Solution::is_power_of_two_v1(16), true);
    assert_eq!(Solution::is_power_of_two_v1(3), false);
    assert_eq!(Solution::is_power_of_two_v1(4), true);
    assert_eq!(Solution::is_power_of_two_v1(5), false);
}
