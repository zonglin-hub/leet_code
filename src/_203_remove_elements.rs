//! 移除链表元素
//!
//! 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。

#![allow(unused)]

use crate::{ListNode, Solution};

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
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

#[cfg(test)]
mod tests {
    use crate::create_list;

    use super::*;

    #[test]
    fn test_remove_elements() {
        // 输入：head = [1,2,6,3,4,5,6], val = 6
        // 输出：[1,2,3,4,5]
        assert_eq!(
            Solution::remove_elements(create_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
            create_list(vec![1, 2, 3, 4, 5])
        );

        // 输入：head = [], val = 1
        // 输出：[]
        assert_eq!(Solution::remove_elements(None, 1), None);

        // 输入：head = [7,7,7,7], val = 7、
        // 输出：[]
        assert_eq!(
            Solution::remove_elements(create_list(vec![7, 7, 7, 7]), 7),
            None
        );
    }
}
