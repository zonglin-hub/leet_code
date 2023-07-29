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
    /// https://leetcode.cn/problems/path-sum/
    ///
    /// 路径总和
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(node) => {
                let node = node.borrow_mut();
                // if node.left.is_none() && node.right.is_none() {
                //     return target_sum == node.val;
                // }
                node.left.is_none() && node.right.is_none() && target_sum == node.val
                    || Self::has_path_sum(node.left.clone(), target_sum - node.val)
                    || Self::has_path_sum(node.right.clone(), target_sum - node.val)
            }
        }
    }
}

pub fn create_tree_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
