// https://leetcode.cn/problems/merge-two-binary-trees/

#![allow(unused)]
pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// 合并二叉树
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
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
                new_val.left = Self::merge_trees(r1.left.take(), r2.left.take());
                new_val.right = Self::merge_trees(r1.right.take(), r2.right.take());
                Some(Rc::new(RefCell::new(new_val)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn create_tree_node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    /// 输入：root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
    /// 输出：[3,4,5,5,4,null,7]
    #[test]
    fn test_merge_trees() {
        let root1 = create_tree_node(
            1,
            create_tree_node(3, create_tree_node(5, None, None), None),
            create_tree_node(2, None, None),
        );
        let root2 = create_tree_node(
            2,
            create_tree_node(1, None, create_tree_node(4, None, None)),
            create_tree_node(3, None, create_tree_node(7, None, None)),
        );
        let merged = Solution::merge_trees(root1, root2);
        let expected = create_tree_node(
            3,
            create_tree_node(
                4,
                create_tree_node(5, None, None),
                create_tree_node(4, None, None),
            ),
            create_tree_node(5, None, create_tree_node(7, None, None)),
        );
        assert_eq!(merged, expected);
    }

    #[test]
    fn test_merge_trees1() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let merged = Solution::merge_trees(root1, root2);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let expected = create_tree_node(2, None, create_tree_node(2, None, None));

        assert_eq!(merged, expected);
    }
}
