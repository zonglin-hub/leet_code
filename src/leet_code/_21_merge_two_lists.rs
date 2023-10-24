//! 合并两个有序链表
//!
//! 输入两个递增排序的链表，合并这两个链表并使新链表中的节点仍然是递增排序的。

use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    pub fn merge_two_lists_v1(list1: ListNodePtr, list2: ListNodePtr) -> ListNodePtr {
        fn carried(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
            if l1.is_none() && l2.is_none() {
                return None;
            }
            match (l1, l2) {
                (None, None) => None,
                (None, r) => r,
                (l, None) => l,
                (Some(mut l), Some(mut r)) => match l.val <= r.val {
                    true => {
                        l.next = carried(l.next, Some(r));
                        Some(l)
                    }
                    false => {
                        r.next = carried(Some(l), r.next);
                        Some(r)
                    }
                },
            }
        }
        carried(list1, list2)
    }
}

impl Solution {
    pub fn merge_two_lists(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
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
