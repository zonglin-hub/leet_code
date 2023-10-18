use leet_code::leet_code::Solution;

#[test]
fn test_combination_sum2() {
    assert_eq!(
        Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
    );
    assert_eq!(
        Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
        vec![vec![1, 2, 2], vec![5]]
    );
}
