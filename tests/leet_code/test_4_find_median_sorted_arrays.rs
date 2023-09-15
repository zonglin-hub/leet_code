use leet_code::Solution;

#[test]
fn test_find_median_sorted_arrays_v1() {
    assert_eq!(
        Solution::find_median_sorted_arrays_v1(vec![1, 3], vec![2]),
        2.00000
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_v1(vec![1, 2], vec![3, 4]),
        2.50000
    );
}
