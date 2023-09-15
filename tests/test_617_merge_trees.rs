use std::{cell::RefCell, rc::Rc};

use leet_code::{create_tree_node, Solution, TreeNode};

#[test]
fn test_merge_trees_v1() {
    assert_eq!(
        Solution::merge_trees_v1(
            create_tree_node(
                1,
                create_tree_node(3, create_tree_node(5, None, None), None),
                create_tree_node(2, None, None),
            ),
            create_tree_node(
                2,
                create_tree_node(1, None, create_tree_node(4, None, None)),
                create_tree_node(3, None, create_tree_node(7, None, None)),
            )
        ),
        create_tree_node(
            3,
            create_tree_node(
                4,
                create_tree_node(5, None, None),
                create_tree_node(4, None, None),
            ),
            create_tree_node(5, None, create_tree_node(7, None, None)),
        )
    );
    assert_eq!(
        Solution::merge_trees_v1(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
        ),
        create_tree_node(2, None, create_tree_node(2, None, None))
    );
}
