//! 将有序数组转换为二叉搜索树
//!
//! 给你一个整数数组 nums ，其中元素已经按 升序 排列，请你将其转换为一棵 高度平衡 二叉搜索树。
//！
//！ 高度平衡 二叉树是一棵满足「每个节点的左右两个子树的高度差的绝对值不超过 1 」的二叉树。
//!

use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// 递归
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::sorted_array_to_bst_helper(nums)
    }

    /// 将一个有序数组转换为二叉搜索树。
    fn sorted_array_to_bst_helper(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 如果数组为空，则返回None
        if nums.is_empty() {
            return None;
        }

        // 计算数组中间位置的索引
        let mid = nums.len() / 2;

        // 创建根节点，并将其赋值给node
        let mut node = TreeNode::new(nums[mid]);

        // 将左子树赋值给node.left
        node.left = Solution::sorted_array_to_bst_helper(nums[0..mid].to_vec());

        // 将右子树赋值给node.right
        node.right = Solution::sorted_array_to_bst_helper(nums[mid + 1..].to_vec());

        // 返回Rc::new(RefCell::new(node))
        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_array_to_bst() {
        // 输入：nums = [-10,-3,0,5,9]
        // 输出：[0,-3,9,-10,null,5]
        // 解释：[0,-10,5,null,-3,null,9]
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

        // 输入：nums = [1,3]
        // 输出：[3,1]
        // 解释：[1,null,3] 和 [3,1] 都是高度平衡二叉搜索树。
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
}
