use leet_code::{create_list, Solution};

#[test]
fn test_swap_pairs_v1() {
    assert_eq!(
        Solution::swap_pairs_v1(create_list(vec![1, 2, 3, 4])),
        create_list(vec![2, 1, 4, 3])
    );
    assert_eq!(
        Solution::swap_pairs_v1(create_list(vec![])),
        create_list(vec![])
    );
    assert_eq!(
        Solution::swap_pairs_v1(create_list(vec![1])),
        create_list(vec![1])
    );
}
