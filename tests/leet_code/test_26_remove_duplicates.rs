use leet_code::leet_code::Solution;

#[test]
fn test_remove_duplicates_v1() {
    assert_eq!(Solution::remove_duplicates_v1(&mut vec![1, 1, 2]), 2);
    assert_eq!(
        Solution::remove_duplicates_v1(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
        5
    );
}
