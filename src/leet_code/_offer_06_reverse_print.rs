//! 剑指 Offer 06. 从尾到头打印链表

use super::{ListNode, Solution};

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        let mut node = &head;

        while let Some(x) = node {
            res.push(x.val);
            node = &x.next;
        }

        res.reverse();
        res
    }
}
