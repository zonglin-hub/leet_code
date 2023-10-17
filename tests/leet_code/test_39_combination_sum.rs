use leet_code::leet_code::Solution;

#[test]
fn test_combination_sum() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![3, 2, 2], vec![7]]
    );
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![3, 3, 2], vec![5, 3]]
    );
    // assert_eq!(
    //     Solution::combination_sum(vec![2], 1),
    //     vec![Vec::<i32>::new()]
    // );
}
