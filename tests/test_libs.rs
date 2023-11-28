use leet_code::leet_code::{array_to_tree, create_tree_node, expected_sort, expected_sort_vec};

#[test]
fn test_create_tree_node() {
    assert_eq!(create_tree_node(1, None, None).unwrap().borrow().val, 1);
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

#[test]
fn test_array_to_tree() {
    let root = vec![
        Some(3),
        Some(2),
        Some(3),
        Some(-1),
        Some(3),
        Some(-1),
        Some(1),
    ];
    let tree = array_to_tree(root);
    println!("Binary Tree: {:#?}", tree);
}
