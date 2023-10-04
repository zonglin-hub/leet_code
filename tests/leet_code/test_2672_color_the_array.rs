use leet_code::leet_code::{expected_sort_vec, Solution};

#[test]
fn test_color_the_array() {
    assert_eq!(
        Solution::color_the_array(
            4,
            expected_sort_vec(vec![[0, 2], [1, 2], [3, 1], [1, 1], [2, 1]])
        ),
        vec![0, 1, 1, 0, 2]
    );

    assert_eq!(
        Solution::color_the_array(1, expected_sort_vec(vec![[0, 100000]])),
        vec![0]
    );
}
