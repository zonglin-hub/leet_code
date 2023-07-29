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
struct Solution;

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
