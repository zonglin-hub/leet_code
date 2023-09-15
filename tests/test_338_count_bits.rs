use leet_code::Solution;

#[test]
fn test_count_bits() {
    assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
