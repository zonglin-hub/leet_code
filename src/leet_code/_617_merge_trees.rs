//! 合并二叉树

use super::{Solution, TreeNode, TreeNodePtr};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// 这个函数实现了合并两棵二叉树的功能。它接受两个二叉树的根节点指针作为输入，并返回一个新的二叉树的根节点指针。
    ///
    /// 在函数内部，它使用模式匹配来处理不同的情况。
    /// 如果两棵二叉树都为空，那么返回空指针。如果其中一棵二叉树为空，那么返回另一棵二叉树的根节点指针。
    /// 如果两棵二叉树都不为空，那么它创建一个新的二叉树节点，并将两棵二叉树的根节点的值相加作为新节点的值。
    /// 然后，它分别获取两棵二叉树的左子节点和右子节点，并使用递归调用`merge_trees_v1`函数来合并它们。
    /// 最后，它将合并后的左子节点和右子节点作为新节点的左子节点和右子节点，并将新节点存储在`Rc`中，返回新节点的指针。
    ///
    /// 通过这种方式，函数就实现了合并两棵二叉树的功能。
    pub fn merge_trees(root1: TreeNodePtr, root2: TreeNodePtr) -> TreeNodePtr {
        match (root1, root2) {
            (None, None) => None,
            (None, r) | (r, None) => r,
            (Some(r1), Some(r2)) => {
                let mut r1 = r1.borrow_mut();
                let mut r2 = r2.borrow_mut();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: r1.val + r2.val,
                    left: Self::merge_trees(r1.left.take(), r2.left.take()),
                    right: Self::merge_trees(r1.right.take(), r2.right.take()),
                })))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::linked_tree;
    use crate::leet_code::Solution;

    #[test]
    fn test_merge_trees() {
        assert_eq!(
            Solution::merge_trees(
                linked_tree(
                    1,
                    linked_tree(3, linked_tree(5, None, None), None),
                    linked_tree(2, None, None)
                ),
                linked_tree(
                    2,
                    linked_tree(1, None, linked_tree(4, None, None)),
                    linked_tree(3, None, linked_tree(7, None, None))
                )
            ),
            linked_tree(
                3,
                linked_tree(4, linked_tree(5, None, None), linked_tree(4, None, None)),
                linked_tree(5, None, linked_tree(7, None, None))
            )
        );

        assert_eq!(
            Solution::merge_trees(
                linked_tree(1, None, None),
                linked_tree(1, None, linked_tree(2, None, None))
            ),
            linked_tree(2, None, linked_tree(2, None, None))
        );
    }
}
