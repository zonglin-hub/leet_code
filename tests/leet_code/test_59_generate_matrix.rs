use leet_code::leet_code::Solution;

#[test]
fn test_length_of_last_word() {
    assert_eq!(
        Solution::generate_matrix(3),
        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    );

    assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
}
