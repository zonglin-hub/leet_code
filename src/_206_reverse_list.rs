// Definition for singly-linked list.
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
    /// 反转链表
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse(head, None)
    }

    fn reverse(head: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let tail = node.next.take();
            node.next = prev;
            return Self::reverse(tail, Some(node));
        }
        prev
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
            p.as_mut().expect("").next = node;
            // 将p的下一个节点赋值给p
            p = p.expect("").next.as_mut();
        }
        // 返回head
        head
    }

    #[test]
    fn test_reverse_list() {
        // 输入：head = [1,2,3,4,5]
        // 输出：[5,4,3,2,1]
        assert_eq!(
            Solution::reverse_list(create_list(vec![1, 2, 3, 4, 5])),
            create_list(vec![5, 4, 3, 2, 1])
        );

        // 输入：head = [1,2]
        // 输出：[2,1]
        assert_eq!(
            Solution::reverse_list(create_list(vec![1, 2])),
            create_list(vec![2, 1])
        );

        // 输入：head = []
        // 输出：[]
        assert_eq!(Solution::reverse_list(None), None);
    }
}
