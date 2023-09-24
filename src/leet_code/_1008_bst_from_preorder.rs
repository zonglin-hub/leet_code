use std::cell::RefCell;
use std::rc::Rc;

use super::{Solution, TreeNode};
impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
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
