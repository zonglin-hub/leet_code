//! 合并 K 个升序链表

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::{ListNode, ListNodePtr, Solution};

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
    pub fn merge_k_lists_v1(lists: Vec<ListNodePtr>) -> ListNodePtr {
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

#[cfg(test)]
mod tests {
    use crate::leet_code::{create_list, Solution};

    #[test]
    fn test_merge_k_lists_v1() {
        assert_eq!(
            Solution::merge_k_lists_v1(vec![
                create_list(vec![1, 4, 5]),
                create_list(vec![1, 3, 4]),
                create_list(vec![2, 6]),
            ]),
            create_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists_v1(vec![]), None);
        assert_eq!(Solution::merge_k_lists_v1(vec![None]), None);
    }
}
