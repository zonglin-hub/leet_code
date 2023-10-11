use leet_code::leet_code::Solution;

#[test]
fn test_min_capability() {
    assert_eq!(Solution::min_capability_2560(vec![2, 3, 5, 9], 2), 5);
    assert_eq!(Solution::min_capability_2560(vec![2, 7, 9, 3, 1], 2), 2);
}
