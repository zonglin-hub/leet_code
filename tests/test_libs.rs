use leet_code::{create_tree_node, expected_sort, expected_sort_vec};

#[test]
fn test_create_tree_node() {
    let node = create_tree_node(1, None, None);
    assert_eq!(node.expect("").borrow().val, 1);
}

#[test]
fn test_expected_sort() {
    assert_eq!(
        expected_sort(vec![vec![0, 1], vec![3, 3], vec![1, 0]]),
        vec![[0, 1], [1, 0], [3, 3]]
    );
}

#[test]
fn test_expected_sort_vec() {
    assert_eq!(
        expected_sort_vec(vec![[0, 1], [1, 0], [3, 3]]),
        vec![[0, 1], [1, 0], [3, 3]]
    );
}
