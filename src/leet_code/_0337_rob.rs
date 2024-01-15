//! 打家劫舍 III

use super::{Solution, TreeNodePtr};

impl Solution {
    /// 这段代码实现了一个计算二叉树中最大子节点值的函数`rob_337`。它接受一个二叉树的根节点指针作为输入，并返回一个包含最大子节点值的数组。
    ///
    /// 首先，它使用一个函数`rob`来计算二叉树中每个节点的最大子节点值。
    /// 该函数接受一个二叉树的根节点指针作为输入，并返回一个包含左子节点和右子节点的最大子节点值的数组。
    /// 在函数内部，它使用一个变量`dp_l`和`dp_r`来存储左子节点和右子节点的最大子节点值。然后，它使用模式匹配来检查当前节点是否存在。
    /// 如果存在，它将左子节点和右子节点的最大子节点值相加，并与当前节点的值进行比较，取最大值作为当前节点的最大子节点值。
    /// 如果不存在，它将左子节点和右子节点的最大子节点值作为当前节点的最大子节点值。
    ///
    /// 最后，它使用`rob`函数来计算二叉树的根节点的最大子节点值，并返回最大子节点值的数组。这样，函数就实现了计算二叉树中最大子节点值的功能。
    pub fn rob_337(root: TreeNodePtr) -> i32 {
        fn rob(root: &TreeNodePtr) -> [i32; 2] {
            if let Some(n) = root {
                let dp_l = rob(&n.borrow().left);
                let dp_r = rob(&n.borrow().right);
                [dp_l[1] + dp_r[1], (dp_l[0] + dp_r[0] + n.borrow().val).max(dp_l[1] + dp_r[1])]
            } else {
                [0; 2]
            }
        }

        rob(&root)[1]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_rob_337() {
        assert_eq!(
            Solution::rob_337(linked_tree(
                3,
                linked_tree(2, None, linked_tree(3, None, None)),
                linked_tree(3, None, linked_tree(1, None, None))
            )),
            7
        );
    }
}
