use leet_code::leet_code::{expected_sort_vec, Solution};

#[test]
fn test_give_gem() {
    assert_eq!(
        Solution::give_gem(
            vec![3, 1, 2],
            expected_sort_vec(vec![[0, 2], [2, 1], [2, 0]])
        ),
        2
    );
    assert_eq!(
        Solution::give_gem(
            vec![100, 0, 50, 100],
            expected_sort_vec(vec![[0, 2], [0, 1], [3, 0], [3, 0]])
        ),
        75
    );
    assert_eq!(
        Solution::give_gem(
            vec![0, 0, 0, 0],
            expected_sort_vec(vec![[1, 2], [3, 1], [1, 2]])
        ),
        0
    );
}
