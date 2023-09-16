use std::{cell::RefCell, rc::Rc};

use leet_code::leet_code::{Solution, TreeNode};

#[test]
fn test_build_tree() {
    assert_eq!(
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    );

    assert_eq!(
        Solution::build_tree(vec![-1], vec![-1]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: None,
        }))),
    );
}
