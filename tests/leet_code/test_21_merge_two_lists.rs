use leet_code::leet_code::{create_list, Solution};

#[test]
fn test_merge_two_lists_v1() {
    assert_eq!(
        Solution::merge_two_lists_v1(create_list(vec![1, 2, 4]), create_list(vec![1, 3, 4])),
        create_list(vec![1, 1, 2, 3, 4, 4])
    );
    assert_eq!(
        Solution::merge_two_lists_v1(create_list(vec![]), create_list(vec![])),
        create_list(vec![])
    );
    assert_eq!(
        Solution::merge_two_lists_v1(create_list(vec![]), create_list(vec![0])),
        create_list(vec![0])
    );
}

#[test]
fn test_merge_two_lists() {
    assert_eq!(
        Solution::merge_two_lists(create_list(vec![1, 2, 4]), create_list(vec![1, 3, 4])),
        create_list(vec![1, 1, 2, 3, 4, 4])
    );
    assert_eq!(
        Solution::merge_two_lists(create_list(vec![]), create_list(vec![])),
        create_list(vec![])
    );
    assert_eq!(
        Solution::merge_two_lists(create_list(vec![]), create_list(vec![0])),
        create_list(vec![0])
    );
}
