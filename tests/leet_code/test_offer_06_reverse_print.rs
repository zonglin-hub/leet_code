use leet_code::leet_code::{create_list, Solution};

#[test]
fn test_reverse_print() {
    assert_eq!(
        Solution::reverse_print(create_list(vec![1, 3, 2])),
        vec![2, 3, 1]
    );
}
