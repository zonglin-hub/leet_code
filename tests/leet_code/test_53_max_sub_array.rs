use leet_code::Solution;

#[test]
fn test_max_sub_array_v1() {
    assert_eq!(
        Solution::max_sub_array_v1(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(Solution::max_sub_array_v1(vec![1]), 1);
    assert_eq!(Solution::max_sub_array_v1(vec![5, 4, -1, 7, 8]), 23);
}

#[test]
fn test_max_sub_array_v2() {
    assert_eq!(
        Solution::max_sub_array_v2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(Solution::max_sub_array_v2(vec![1]), 1);
    assert_eq!(Solution::max_sub_array_v2(vec![5, 4, -1, 7, 8]), 23);
}
