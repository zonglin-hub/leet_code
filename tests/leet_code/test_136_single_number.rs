use leet_code::leet_code::Solution;

#[test]
fn test_single_number() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    assert_eq!(Solution::single_number(vec![1]), 1);
    assert_eq!(Solution::single_number(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    assert_eq!(Solution::single_number(vec![3, 3, 7, 7, 10, 11, 11]), 10);
}
