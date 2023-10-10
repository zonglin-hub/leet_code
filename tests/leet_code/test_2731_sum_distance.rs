use leet_code::leet_code::Solution;

#[test]
fn test_color_the_array() {
    assert_eq!(
        Solution::sum_distance(vec![-2, 0, 2], "RLL".to_string(), 3),
        8
    );
    assert_eq!(Solution::sum_distance(vec![1, 0], "RL".to_string(), 2), 5);
}
