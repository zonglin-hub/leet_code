use leet_code::leet_code::{expected_sort_vec, Solution};

#[test]
fn test_largest_integer() {
    assert_eq!(
        Solution::collect_the_coins(
            vec![1, 0, 0, 0, 0, 1],
            expected_sort_vec(vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]])
        ),
        2
    );
    assert_eq!(
        Solution::collect_the_coins(
            vec![0, 0, 0, 1, 1, 0, 0, 1],
            expected_sort_vec(vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [5, 6], [5, 7]])
        ),
        2
    );
}
