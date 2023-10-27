use leet_code::leet_code::Solution;

#[test]
fn test_55_can_jump() {
    assert_eq!(
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
    assert_eq!(
        Solution::merge(vec![vec![1, 4], vec![4, 5]]),
        vec![vec![1, 5]]
    );
}
