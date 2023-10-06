//! 移除链表元素
//!
//! 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn remove_elements(head: ListNodePtr, val: i32) -> ListNodePtr {
        // 判断head是否为None，如果为None，则返回None
        match head {
            None => None,
            // 如果head不为None，则判断head的值是否等于val，如果等于，则调用remove_elements函数，并将结果返回
            Some(mut node) => match node.val == val {
                true => Self::remove_elements(node.next, val),
                false => {
                    node.next = Self::remove_elements(node.next, val);
                    Some(node)
                }
            },
        }
    }
}
