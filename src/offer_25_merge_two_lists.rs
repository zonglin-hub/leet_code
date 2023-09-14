#![allow(unused)]
use crate::types::base_type::{ListNode, Solution};

impl Solution {
    /// 合并两个排序的链表
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n), None) | (None, Some(n)) => Some(n),
            (None, None) => None,
            (Some(l1), Some(l2)) => match l1.val >= l2.val {
                true => Some(Box::new(ListNode {
                    val: l2.val,
                    next: Self::merge_two_lists(Some(l1), l2.next),
                })),
                false => Some(Box::new(ListNode {
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2)),
                })),
            },
        }
    }
}
