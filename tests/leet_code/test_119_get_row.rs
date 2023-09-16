use leet_code::leet_code::Solution;

#[test]
fn test() {
    assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    assert_eq!(Solution::get_row(0), vec![1]);
    assert_eq!(Solution::get_row(1), vec![1, 1]);
}
