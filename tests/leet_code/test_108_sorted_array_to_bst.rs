use std::{cell::RefCell, rc::Rc};

use leet_code::leet_code::{Solution, TreeNode};

#[test]
fn test_sorted_array_to_bst() {
    assert_eq!(
        Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -10,
                    left: None,
                    right: None
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
                right: None,
            }))),
        })))
    );
    assert_eq!(
        Solution::sorted_array_to_bst(vec![1, 3]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: None,
        })))
    );
}
