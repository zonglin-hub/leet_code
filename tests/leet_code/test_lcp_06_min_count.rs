use leet_code::leet_code::Solution;

#[test]
fn test_min_count() {
    assert_eq!(Solution::min_count(vec![4, 2, 1]), 4);
    assert_eq!(Solution::min_count(vec![2, 3, 10]), 8);
}
