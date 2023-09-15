//! 合并 K 个升序链表

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::{ListNode, Solution};

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists_v1(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::new();
        for i in lists {
            min_heap.push(i);
        }

        let mut dummy = ListNode::new(-1);
        let mut ptr = &mut dummy;

        while !min_heap.is_empty() {
            let mut node = min_heap.pop().expect("");
            if let Some(n) = node.as_mut() {
                min_heap.push(n.next.take());
                ptr.next = node;
                ptr = ptr.next.as_mut().expect("");
            }
        }
        dummy.next
    }
}
