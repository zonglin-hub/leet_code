//! 路径总和
//!
//! 给你二叉树的根节点 root 和一个表示目标和的整数 targetSum 。判断该树中是否存在 根节点到叶子节点 的路径，这条路径上所有节点值相加等于目标和 targetSum 。如果存在，返回 true ；否则，返回 false 。
//!
//! 叶子节点 是指没有子节点的节点。

use super::{Solution, TreeNodePtr};

impl Solution {
    /// 这段代码实现了一个判断二叉树中是否存在一条路径，使得路径上所有节点的和等于给定的目标和的函数`has_path_sum`。它接受一个二叉树的根节点指针和一个目标和作为输入，并返回一个布尔值，表示是否存在这样的路径。
    ///
    /// 首先，它使用模式匹配来处理根节点指针。如果根节点指针为空，那么表示二叉树为空，因此直接返回`false`。否则，它创建一个局部变量`node`，并将其设置为根节点。
    ///
    /// 然后，它使用模式匹配来检查当前节点是否满足条件。
    /// 如果当前节点的左子节点为空并且右子节点为空，并且目标和等于当前节点的值，那么表示存在一条路径，使得路径上所有节点的和等于目标和，因此直接返回`true`。
    /// 否则，它分别递归地调用`Self::has_path_sum`函数，将左子节点和右子节点作为参数，并将目标和减去当前节点的值作为参数。如果递归调用的结果为`true`，那么表示存在一条路径，使得路径上所有节点的和等于目标和，因此直接返回`true`。
    /// 如果递归调用的结果为`false`，那么表示不存在这样的路径，因此返回`false`。
    ///
    /// 最后，它返回`true`或`false`，表示是否存在这样的路径。这样，函数就实现了判断二叉树中是否存在一条路径，使得路径上所有节点的和等于给定的目标和的功能。
    pub fn has_path_sum(root: TreeNodePtr, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(node) => {
                let node = node.borrow_mut();
                node.left.is_none() && node.right.is_none() && target_sum == node.val
                    || Self::has_path_sum(node.left.clone(), target_sum - node.val)
                    || Self::has_path_sum(node.right.clone(), target_sum - node.val)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{create_tree_node, Solution};

    #[test]
    fn test_has_path_sum() {
        assert_eq!(
            Solution::has_path_sum(
                create_tree_node(
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
                ),
                22,
            ),
            true
        );
    }
}
