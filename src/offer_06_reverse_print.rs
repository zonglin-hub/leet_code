// Definition for singly-linked list.
#![allow(unused)]
use crate::types::base_type::Solution;

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
impl Solution {
    /// 从尾到头打印链表
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let (mut res, mut node) = (vec![], &head);
        while let Some(x) = node {
            res.push(x.val);
            node = &x.next;
        }
        res.reverse();
        res
    }
}
