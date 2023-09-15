use leet_code::{create_list, Solution};

#[test]
fn test_reverse_k_group_v1() {
    assert_eq!(
        Solution::reverse_k_group_v1(create_list(vec![1, 2, 3, 4, 5]), 2),
        create_list(vec![2, 1, 4, 3, 5])
    );
    assert_eq!(
        Solution::reverse_k_group_v1(create_list(vec![1, 2, 3, 4, 5]), 3),
        create_list(vec![3, 2, 1, 4, 5])
    );
}
