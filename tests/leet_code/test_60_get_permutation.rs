use leet_code::leet_code::Solution;

#[test]
fn test_get_permutation() {
    assert_eq!(Solution::get_permutation(3, 3), "213");
    assert_eq!(Solution::get_permutation(4, 9), "2314");
    assert_eq!(Solution::get_permutation(3, 1), "123");
}
