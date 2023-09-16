use leet_code::leet_code::{create_list, Solution};

#[test]
fn test_merge_k_lists_v1() {
    assert_eq!(
        Solution::merge_k_lists_v1(vec![
            create_list(vec![1, 4, 5]),
            create_list(vec![1, 3, 4]),
            create_list(vec![2, 6]),
        ]),
        create_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
    );
    assert_eq!(Solution::merge_k_lists_v1(vec![]), None);
    assert_eq!(Solution::merge_k_lists_v1(vec![None]), None);
}
