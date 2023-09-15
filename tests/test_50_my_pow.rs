use leet_code::Solution;

#[test]
fn test_my_pow() {
    assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
    assert_eq!(Solution::my_pow(2.10000, 3), 9.261000000000001);
    assert_eq!(Solution::my_pow(2.00000, -2), 0.25000);
}
