use leet_code::leet_code::Solution;

#[test]
fn test_jump() {
    assert_eq!(
        Solution::permute(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2]
        ]
    );

    assert_eq!(Solution::permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]]);
    assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
}
