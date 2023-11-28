//! 将有序数组转换为二叉搜索树

use super::{Solution, TreeNode, TreeNodePtr};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// 这个函数是用于将一个有序数组转换为一个高度平衡的二叉搜索树（Balanced Binary Search Tree，BST）。
    ///
    /// 函数的参数是一个整数数组 `nums`，返回值是二叉搜索树的根节点指针。函数内部定义了一个嵌套辅助函数 `sorted_array_to_bst_helper`，这个函数实现了将有序数组转换为平衡二叉搜索树的核心逻辑。
    ///
    /// 函数的主要步骤如下：
    ///
    /// 1. 首先检查输入的数组 `nums` 是否为空，如果为空则返回 `None`，表示当前子树为空。
    /// 2. 如果数组不为空，找到数组的中点位置 `mid`，作为当前子树的根节点值。
    /// 3. 创建一个新的二叉树节点 `node`，并将根节点值设置为 `nums[mid]`。
    /// 4. 递归调用 `sorted_array_to_bst_helper` 函数构建当前节点的左子树，传入左子树部分的数组（从索引 `0` 到 `mid-1`）。
    /// 5. 递归调用 `sorted_array_to_bst_helper` 函数构建当前节点的右子树，传入右子树部分的数组（从索引 `mid+1` 到末尾）。
    /// 6. 最后，将当前节点包装为 `Rc` 和 `RefCell` 类型，并返回根节点的指针。
    ///
    /// 这样，通过递归的方式不断将有序数组划分为左子树和右子树，并选取中点作为根节点，最终可以构建出一个高度平衡的二叉搜索树。
    ///
    /// 最后，在主函数 `sorted_array_to_bst` 中调用辅助函数 `sorted_array_to_bst_helper` 来启动递归过程，并返回构建好的二叉搜索树的根节点指针。
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> TreeNodePtr {
        fn sorted_array_to_bst_helper(nums: Vec<i32>) -> TreeNodePtr {
            if nums.is_empty() {
                return None;
            }

            let mid = nums.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: sorted_array_to_bst_helper(nums[0..mid].to_vec()),
                right: sorted_array_to_bst_helper(nums[mid + 1..].to_vec()),
            })))
        }
        sorted_array_to_bst_helper(nums)
    }
}

#[cfg(test)]
mod tests {

    use crate::leet_code::{Solution, TreeNode};
    use std::{cell::RefCell, rc::Rc};

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
}
