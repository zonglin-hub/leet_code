use leet_code::Solution;

#[test]
fn test_find_kth_bit() {
    assert_eq!(Solution::find_kth_bit(3, 1), '0');
    assert_eq!(Solution::find_kth_bit(4, 11), '1');
    assert_eq!(Solution::find_kth_bit(1, 1), '0');
    assert_eq!(Solution::find_kth_bit(2, 3), '1');
}
