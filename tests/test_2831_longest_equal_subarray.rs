use leet_code::Solution;

#[test]
fn test_longest_equal_subarray() {
    assert_eq!(
        Solution::longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3),
        3
    );
    assert_eq!(
        Solution::longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2),
        4
    );
}
