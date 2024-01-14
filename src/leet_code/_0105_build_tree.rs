//! 从前序与中序遍历序列构造二叉树

use super::{Solution, TreeNode, TreeNodePtr};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// 这个函数是用于构建二叉树的，它根据给定的先序遍历和中序遍历的结果来构建二叉树。函数接受两个参数，`preorder` 是先序遍历结果的整数数组，`inorder` 是中序遍历结果的整数数组。返回值是构建的二叉树的根节点的指针。
    ///
    /// 函数内部定义了一个嵌套函数 `build`，这个函数实际上是实现递归构建二叉树的核心逻辑。它接受两个参数，`inorder` 是中序遍历结果的引用数组，`preorder` 是先序遍历结果的迭代器。返回值是一个元组，包含构建的子树的根节点的指针和剩余的先序遍历迭代器。
    ///
    /// 函数的主要逻辑如下：
    ///
    /// 1. 如果 `inorder` 数组的长度为0，说明当前子树为空，返回 `(None, preorder)`，即根节点为空，迭代器保持不变。
    /// 2. 否则，从 `preorder` 中取出第一个元素作为当前子树的根节点值。
    /// 3. 在 `inorder` 数组中查找根节点的位置 `pos`，将中序遍历数组分为左子树和右子树。
    /// 4. 递归调用 `build` 函数构建左子树和右子树，分别得到左子树和右子树的根节点和剩余的先序遍历迭代器。
    /// 5. 创建一个新的 `TreeNode`，将根节点值、左子树和右子树组装起来，并用 `Rc` 和 `RefCell` 进行包装。
    /// 6. 返回构建的当前子树的根节点的指针和剩余的先序遍历迭代器。
    ///
    /// 最后，调用 `build` 函数构建整个二叉树，并返回根节点的指针。
    ///
    /// 需要注意的是，在构建二叉树的过程中，通过先序遍历数组来确定每个节点的值，通过中序遍历数组来确定左子树和右子树的划分位置。这种方式能够唯一确定一棵二叉树。
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> TreeNodePtr {
        fn build<'a, IT>(inorder: &[i32], mut preorder: IT) -> (TreeNodePtr, IT)
        where
            IT: Iterator<Item = &'a i32>,
        {
            let len = inorder.len();
            if inorder.is_empty() {
                return (None, preorder);
            }

            let root = preorder.next();
            let mut pos = 0;
            while pos < len && inorder[pos] != *root.unwrap() {
                pos += 1;
            }

            let (left, preorder) = build(&inorder[0..pos], preorder);
            let (right, preorder) = build(&inorder[pos + 1..len], preorder);

            (Some(Rc::new(RefCell::new(TreeNode { val: *root.unwrap(), left, right }))), preorder)
        }

        build(&inorder, preorder.iter()).0
    }
}

#[cfg(test)]
mod tests {

    use crate::leet_code::linked_tree;
    use crate::leet_code::Solution;

    #[test]
    fn test_build_tree() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            linked_tree(
                3,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )
        );
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            linked_tree(
                3,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )
        );
        assert_eq!(Solution::build_tree(vec![-1], vec![-1]), linked_tree(-1, None, None));
    }
}
