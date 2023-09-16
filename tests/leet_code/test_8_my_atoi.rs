use leet_code::leet_code::Solution;

#[test]
fn test_my_atoi_v1() {
    assert_eq!(Solution::my_atoi_v1(String::from("42")), 42);
    assert_eq!(Solution::my_atoi_v1(String::from("   -42")), -42);
    assert_eq!(Solution::my_atoi_v1(String::from("4193 with words")), 4193);
}
