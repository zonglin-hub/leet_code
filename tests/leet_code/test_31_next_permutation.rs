use leet_code::leet_code::Solution;

#[test]
fn test_next_permutation() {
    let mut nums = vec![1, 2, 3];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, [1, 3, 2]);
}
