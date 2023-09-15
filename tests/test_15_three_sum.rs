use leet_code::Solution;

#[test]
fn test_three_sum_v1() {
    assert_eq!(
        Solution::three_sum_v1(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
    assert_eq!(
        Solution::three_sum_v1(vec![0, 1, 1]),
        Vec::<Vec<i32>>::new()
    );
    assert_eq!(Solution::three_sum_v1(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
}
