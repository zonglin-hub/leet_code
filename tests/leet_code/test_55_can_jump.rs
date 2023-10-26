use leet_code::leet_code::Solution;

#[test]
fn test_55_can_jump() {
    assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
}
