use leet_code::leet_code::Solution;

#[test]
fn test_rob_2_v2() {
    assert_eq!(Solution::rob_2_v2(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob_2_v2(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob_2_v2(vec![1, 2, 3]), 3);
}

#[test]
fn test_rob_2_v1() {
    assert_eq!(Solution::rob_2_v1(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob_2_v1(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob_2_v1(vec![1, 2, 3]), 3);
}
