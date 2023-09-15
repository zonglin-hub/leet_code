use leet_code::{create_list, Solution};

#[test]
fn test_remove_nth_from_end_v1() {
    assert_eq!(
        Solution::remove_nth_from_end_v1(create_list(vec![1, 2, 3, 4, 5]), 2),
        create_list(vec![1, 2, 3, 5])
    );
    assert_eq!(
        Solution::remove_nth_from_end_v1(create_list(vec![1]), 1),
        None
    );
    assert_eq!(
        Solution::remove_nth_from_end_v1(create_list(vec![1, 2]), 1),
        create_list(vec![1])
    );
}

#[test]
fn test_remove_nth_from_end_v2() {
    assert_eq!(
        Solution::remove_nth_from_end_v2(create_list(vec![1, 2, 3, 4, 5]), 2),
        create_list(vec![1, 2, 3, 5])
    );
    assert_eq!(
        Solution::remove_nth_from_end_v2(create_list(vec![1]), 1),
        None
    );
    assert_eq!(
        Solution::remove_nth_from_end_v2(create_list(vec![1, 2]), 1),
        create_list(vec![1])
    );
}
