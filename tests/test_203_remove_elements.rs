use leet_code::{create_list, Solution};

#[test]
fn test_remove_elements() {
    assert_eq!(
        Solution::remove_elements(create_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
        create_list(vec![1, 2, 3, 4, 5])
    );
    assert_eq!(Solution::remove_elements(None, 1), None);
    assert_eq!(
        Solution::remove_elements(create_list(vec![7, 7, 7, 7]), 7),
        None
    );
}
