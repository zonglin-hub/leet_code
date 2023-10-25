use leet_code::leet_code::Solution;

#[test]
fn test_my_pow() {
    assert_eq!(Solution::total_n_queens(1), 1);
    assert_eq!(Solution::total_n_queens(4), 2);
}
