use leet_code::leet_code::Solution;

#[test]
fn test_is_power_of_two_v1() {
    assert_eq!(Solution::is_power_of_two_v1(1), true);
    assert_eq!(Solution::is_power_of_two_v1(16), true);
    assert_eq!(Solution::is_power_of_two_v1(3), false);
    assert_eq!(Solution::is_power_of_two_v1(4), true);
    assert_eq!(Solution::is_power_of_two_v1(5), false);
}

#[test]
fn test_is_power_of_three() {
    assert!(Solution::is_power_of_three_326(27));
    assert_eq!(Solution::is_power_of_three_326(0), false);
    assert!(Solution::is_power_of_three_326(9));
    assert_eq!(Solution::is_power_of_three_326(45), false);
}

#[test]
fn test_is_power_of_four() {
    assert!(Solution::is_power_of_four_342(16));
    assert!(!Solution::is_power_of_four_342(5));
    assert!(Solution::is_power_of_four_342(1));
}
