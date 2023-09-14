//! 合并两个有序链表

#![allow(unused)]

use crate::types::base_type::Solution;

// pub struct Solution;

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
    pub fn merge_two_lists_v1(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::carried_v1(list1, list2)
    }

    fn carried_v1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => match l.val <= r.val {
                true => {
                    l.next = Self::carried_v1(l.next, Some(r));
                    Some(l)
                }
                false => {
                    r.next = Self::carried_v1(Some(l), r.next);
                    Some(r)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        if nums.is_empty() {
            return None;
        }
        let mut head = Some(Box::new(ListNode::new(nums[0])));
        let mut p = head.as_mut();
        for num in nums.iter().skip(1) {
            let node = Some(Box::new(ListNode::new(*num)));
            p.as_mut().expect("").next = node;
            p = p.expect("").next.as_mut();
        }
        head
    }

    #[test]
    fn test_merge_two_lists_v1() {
        /*
            输入：l1 = [1,2,4], l2 = [1,3,4]
            输出：[1,1,2,3,4,4]
        */
        assert_eq!(
            Solution::merge_two_lists_v1(create_list(vec![1, 2, 4]), create_list(vec![1, 3, 4])),
            create_list(vec![1, 1, 2, 3, 4, 4])
        );

        /*
            输入：l1 = [], l2 = []
            输出：[]
        */
        assert_eq!(
            Solution::merge_two_lists_v1(create_list(vec![]), create_list(vec![])),
            create_list(vec![])
        );
        // assert_eq!(
        //     Solution::merge_two_lists_v1(create_list(vec![]), create_list(vec![])),
        //     create_list(vec![])
        // );

        /*
            输入：l1 = [], l2 = [0]
            输出：[0]
        */
        assert_eq!(
            Solution::merge_two_lists_v1(create_list(vec![]), create_list(vec![0])),
            create_list(vec![0])
        );
    }
}
