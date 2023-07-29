use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution;

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

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    /// https://leetcode.cn/problems/merge-k-sorted-lists/
    ///
    /// 合并 K 个升序链表
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::new();
        for i in lists {
            min_heap.push(i);
        }

        let mut dummy = ListNode::new(-1);
        let mut ptr = &mut dummy;

        while !min_heap.is_empty() {
            let mut node = min_heap.pop().unwrap();
            if let Some(n) = node.as_mut() {
                min_heap.push(n.next.take());
                ptr.next = node;
                ptr = ptr.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    pub fn create_tree_node(nums: i32) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: nums,
            next: None,
        }))
    }

    pub fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = create_tree_node(nums[0]);
        let mut p = head.as_mut();
        for i in nums.iter().skip(1) {
            let node = create_tree_node(*i);
            p.as_mut().unwrap().next = node;
            p = p.unwrap().next.as_mut();
        }
        head
    }

    #[test]
    fn test_merge_k_lists() {
        let expectation = vec![
            create_list(vec![1, 4, 5]),
            create_list(vec![1, 3, 4]),
            create_list(vec![2, 6]),
        ];

        let expect: Option<Box<ListNode>> = create_list(vec![1, 1, 2, 3, 4, 4, 5, 6]);

        assert_eq!(Solution::merge_k_lists(expectation), expect);

        let expectation: Option<Box<ListNode>> = None;
        assert_eq!(Solution::merge_k_lists(vec![]), expectation);
        assert_eq!(Solution::merge_k_lists(vec![None]), expectation);
    }
}
