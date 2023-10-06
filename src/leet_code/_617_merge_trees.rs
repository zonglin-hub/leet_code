//! 合并二叉树

use super::{Solution, TreeNode, TreeNodePtr};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn merge_trees_v1(root1: TreeNodePtr, root2: TreeNodePtr) -> TreeNodePtr {
        match (root1, root2) {
            (None, None) => None,
            (None, r) | (r, None) => r,
            // (Some(r1), Some(r2)) => match (r1.borrow_mut(), r2.borrow_mut()) {
            //     (mut r1, mut r2) => Some(Rc::new(RefCell::new(TreeNode {
            //         val: r1.val + r2.val,
            //         left: Self::merge_trees(r1.left.take(), r2.left.take()),
            //         right: Self::merge_trees(r1.right.take(), r2.right.take()),
            //     }))),
            // },
            (Some(r1), Some(r2)) => {
                let mut r1 = r1.borrow_mut();
                let mut r2 = r2.borrow_mut();
                let mut new_val = TreeNode::new(r1.val + r2.val);
                new_val.left = Self::merge_trees_v1(r1.left.take(), r2.left.take());
                new_val.right = Self::merge_trees_v1(r1.right.take(), r2.right.take());
                Some(Rc::new(RefCell::new(new_val)))
            }
        }
    }
}
