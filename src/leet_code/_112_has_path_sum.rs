//! 路径总和
//!
//! 给你二叉树的根节点 root 和一个表示目标和的整数 targetSum 。判断该树中是否存在 根节点到叶子节点 的路径，这条路径上所有节点值相加等于目标和 targetSum 。如果存在，返回 true ；否则，返回 false 。
//!
//! 叶子节点 是指没有子节点的节点。

use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn has_path_sum(root: TreeNodePtr, target_sum: i32) -> bool {
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
