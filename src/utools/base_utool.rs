//! 基础工具

use crate::types::base_type::{ListNode, TreeNode};

use std::{cell::RefCell, rc::Rc};

pub fn create_tree_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

pub fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    if nums.is_empty() {
        return None;
    }

    // 创建一个头节点，并将其赋值给head
    let mut head = Some(Box::new(ListNode::new(nums[0])));
    // 将head赋值给p
    let mut p = head.as_mut();
    // 遍历nums数组，将每一个元素赋值给ListNode
    for num in nums.iter().skip(1) {
        let node = Some(Box::new(ListNode::new(*num)));
        // 将ListNode赋值给p的下一个节点
        p.as_mut().expect("").next = node;
        // 将p的下一个节点赋值给p
        p = p.expect("").next.as_mut();
    }
    // 返回head
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tree_node() {
        let node = create_tree_node(1, None, None);
        assert_eq!(node.expect("").borrow().val, 1);
    }
}
