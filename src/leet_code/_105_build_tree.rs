//! 从前序与中序遍历序列构造二叉树

use super::{Solution, TreeNode, TreeNodePtr};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> TreeNodePtr {
        fn build<'a, IT>(inorder: &[i32], mut preorder: IT) -> (TreeNodePtr, IT)
        where
            IT: Iterator,
            <IT as Iterator>::Item: Into<&'a i32>,
        {
            let len = inorder.len();
            if len == 0 {
                (None, preorder)
            } else {
                let root = preorder.next().expect("").into();
                let mut pos = 0;

                loop {
                    if inorder[pos] == *root {
                        break;
                    }
                    pos += 1;
                }
                let (left, preorder) = build(&inorder[0..pos], preorder);
                let (right, preorder) = build(&inorder[pos + 1..len], preorder);

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

        build(&inorder, preorder.iter()).0
    }
}
