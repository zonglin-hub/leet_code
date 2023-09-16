//! 从前序与中序遍历序列构造二叉树
//!

use std::cell::RefCell;
use std::rc::Rc;

use super::{Solution, TreeNode};

// use super::{Solution, TreeNode};

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&inorder, preorder.iter()).0
    }

    fn build<'a, IT>(inorder: &[i32], mut preorder: IT) -> (Option<Rc<RefCell<TreeNode>>>, IT)
    where
        IT: Iterator,
        <IT as Iterator>::Item: Into<&'a i32>,
    {
        // 获取树的长度
        let len = inorder.len();
        // 如果树的长度为0，则返回None，否则返回preorder中的第一个元素
        if len == 0 {
            (None, preorder)
        } else {
            // 获取根节点的值
            let root = preorder
                .next()
                .expect("error: Node acquisition failed")
                .into();
            // 初始化pos
            let mut pos = 0;

            // 循环遍历树，找到根节点的位置
            loop {
                // 如果根节点的值等于树中的值，则结束循环
                if inorder[pos] == *root {
                    break;
                }
                // 否则pos加1
                pos += 1;
            }

            // 获取左子树和右子树
            let (left, preorder) = Self::build(&inorder[0..pos], preorder);
            let (right, preorder) = Self::build(&inorder[pos + 1..len], preorder);

            // 返回根节点和前序遍历的结果
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: *root,
                    left,
                    right,
                }))),
                preorder,
            )
        }
    }
}
