use leet_code::leet_code::Solution;

#[test]
fn test_four_sum_v1() {
    assert_eq!(
        Solution::four_sum_v1(vec![1, 0, -1, 0, -2, 2], 0),
        vec![vec![1, 2, -1, -2], vec![0, 2, 0, -2], vec![0, 1, 0, -1]]
    );
    assert_eq!(
        Solution::four_sum_v1(vec![2, 2, 2, 2, 2], 8),
        vec![vec![2, 2, 2, 2]]
    );
}
