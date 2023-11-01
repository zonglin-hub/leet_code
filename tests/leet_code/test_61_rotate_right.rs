use leet_code::leet_code::{create_list, Solution};

#[test]
fn test_rotate_right() {
    assert_eq!(
        Solution::rotate_right(create_list(vec![1, 2, 3, 4, 5]), 3),
        create_list(vec![3, 4, 5, 1, 2])
    );

    assert_eq!(
        Solution::rotate_right(create_list(vec![0, 1, 2]), 4),
        create_list(vec![2, 0, 1])
    );
}
