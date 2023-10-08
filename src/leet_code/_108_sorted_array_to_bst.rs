//! 将有序数组转换为二叉搜索树

use super::{Solution, TreeNode, TreeNodePtr};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// 递归
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> TreeNodePtr {
        Self::sorted_array_to_bst_helper(nums)
    }

    /// 将一个有序数组转换为二叉搜索树。
    fn sorted_array_to_bst_helper(nums: Vec<i32>) -> TreeNodePtr {
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
