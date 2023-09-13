//! 移除链表元素
//!
//! 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。

#![allow(unused)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use crate::types::base_type::Solution;

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
    use super::*;

    pub fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        // 创建一个头节点，并将其赋值给head
        let mut head = Some(Box::new(ListNode::new(nums[0])));
        // 将head赋值给p
        let mut p = head.as_mut();
        // 遍历nums数组，将每一个元素赋值给ListNode
        for num in nums.iter().skip(1) {
            let node = Some(Box::new(ListNode::new(*num)));
            // 将ListNode赋值给p的下一个节点
            p.as_mut().unwrap().next = node;
            // 将p的下一个节点赋值给p
            p = p.unwrap().next.as_mut();
        }
        // 返回head
        head
    }

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
