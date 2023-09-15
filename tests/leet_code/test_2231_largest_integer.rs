use leet_code::Solution;

#[test]
fn test_largest_integer() {
    assert_eq!(Solution::largest_integer(1234), 3412);
    assert_eq!(Solution::largest_integer(65875), 87655);
}
