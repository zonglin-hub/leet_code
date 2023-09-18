use leet_code::leet_code::Solution;

#[test]
fn test_generate_118() {
    assert_eq!(Solution::generate_118(1), vec![vec![1]]);
    assert_eq!(Solution::generate_118(2), vec![vec![1], vec![1, 1]]);
}

#[test]
fn test_get_row_119() {
    assert_eq!(Solution::get_row_119(3), vec![1, 3, 3, 1]);
    assert_eq!(Solution::get_row_119(0), vec![1]);
    assert_eq!(Solution::get_row_119(1), vec![1, 1]);
}
