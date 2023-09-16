use leet_code::leet_code::{create_tree_node, Solution};

#[test]
fn test_merge_trees() {
    let root = create_tree_node(
        5,
        create_tree_node(
            4,
            create_tree_node(
                11,
                create_tree_node(7, None, None),
                create_tree_node(2, None, None),
            ),
            None,
        ),
        create_tree_node(
            8,
            create_tree_node(13, None, None),
            create_tree_node(4, None, create_tree_node(1, None, None)),
        ),
    );
    let merged = Solution::has_path_sum(root, 22);
    assert_eq!(merged, true);
}
