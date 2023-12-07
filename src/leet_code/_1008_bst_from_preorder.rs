//! 前序遍历构造二叉搜索树

use super::{Solution, TreeNode, TreeNodePtr};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// 这个函数实现了从给定的前序遍历序列中构建二叉搜索树的功能。它接受一个包含整数的向量作为输入，并返回一个二叉树的根节点指针。
    ///
    /// 在函数内部，它首先检查输入向量是否为空。
    /// 如果为空，那么它返回空指针。否则，它找到前序遍历序列中的第一个大于根节点值的位置。
    /// 如果找不到这样的位置，那么它将整个序列作为左子树。否则，它将前序遍历序列的前一部分作为左子树，后一部分作为右子树。
    /// 然后，它使用递归调用`bst_from_preorder`函数来构建左子树和右子树，并将它们作为根节点的左子节点和右子节点。最后，它将构建好的根节点存储在`Rc`中，并返回根节点的指针。
    ///
    /// 通过这种方式，函数就实现了从给定的前序遍历序列中构建二叉搜索树的功能。
    pub fn bst_from_preorder(preorder: Vec<i32>) -> TreeNodePtr {
        if preorder.is_empty() {
            return None;
        }

        let devide = preorder
            .iter()
            .position(|&val| val > preorder[0])
            .unwrap_or(preorder.len());

        Some(Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: Self::bst_from_preorder(preorder[1..devide].to_vec()),
            right: Self::bst_from_preorder(preorder[devide..].to_vec()),
        })))
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::linked_tree;
    use crate::leet_code::Solution;

    #[test]
    fn test_bst_from_preorder() {
        assert_eq!(
            Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]),
            linked_tree(
                8,
                linked_tree(5, linked_tree(1, None, None), linked_tree(7, None, None)),
                linked_tree(10, None, linked_tree(12, None, None))
            )
        );
        assert_eq!(
            Solution::bst_from_preorder(vec![1, 3]),
            linked_tree(1, None, linked_tree(3, None, None))
        );
    }
}
