// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}
use crate::leet_code::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_sep = Vec::new();
        Self::get_inorder(&root, &mut inorder_sep);
        Self::build(&inorder_sep, 0, inorder_sep.len() as i32 - 1)
    }

    fn get_inorder(root: &Option<Rc<RefCell<TreeNode>>>, sep: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            Self::get_inorder(&node_ref.left, sep);
            sep.push(node_ref.val);
            Self::get_inorder(&node_ref.right, sep);
        }
    }

    fn build(sep: &[i32], l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            return None;
        }

        let mid = (l + r) >> 1;
        let mut node = TreeNode::new(sep[mid as usize]);
        node.left = Self::build(sep, l, mid - 1);
        node.right = Self::build(sep, mid + 1, r);
        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use crate::tree;

    use super::*;

    #[test]
    fn test_balance_bst() {
        // 构建一个已经平衡的BST
        //       2
        //      / \
        //     1   3
        let root = tree!(2, tree!(1, None, None), tree!(3, None, None));
        assert_eq!(Solution::balance_bst(root.clone()), root);
        // 输入：root = [1,null,2,null,3,null,4,null,null]
        // 输出：[2,1,3,null,null,null,4]
        // 解释：这不是唯一的正确答案，[3,1,4,null,2,null,null] 也是一个可行的构造方案。
    }
}
