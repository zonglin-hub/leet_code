//! 基础工具

use crate::types::base_type::TreeNode;

use std::{cell::RefCell, rc::Rc};

pub fn create_tree_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tree_node() {
        let node = create_tree_node(1, None, None);
        assert_eq!(node.unwrap().borrow().val, 1);
    }
}