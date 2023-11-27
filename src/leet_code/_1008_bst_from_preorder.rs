//! 前序遍历构造二叉搜索树

use super::{Solution, TreeNode, TreeNodePtr};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> TreeNodePtr {
        if !preorder.is_empty() {
            let devide = preorder
                .iter()
                .position(|&val| val > preorder[0])
                .unwrap_or(preorder.len());

            Some(Rc::new(RefCell::new(TreeNode {
                val: preorder[0],
                left: Solution::bst_from_preorder(preorder[1..devide].to_vec()),
                right: Solution::bst_from_preorder(preorder[devide..].to_vec()),
            })))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_bst_from_preorder() {
        assert_eq!(
            Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 12,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        );
        assert_eq!(
            Solution::bst_from_preorder(vec![1, 3]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            })))
        );
    }
}
