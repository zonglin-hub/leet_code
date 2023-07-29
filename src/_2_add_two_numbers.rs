#![allow(unused)]

use std::ops::{Add, Div};

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

impl Solution {
    /// https://leetcode.cn/problems/add-two-numbers/
    ///
    /// 两数相加
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::carried(l1, l2, 0)
    }

    fn carried(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        mut carry: i32,
    ) -> Option<Box<ListNode>> {
        match l1.is_none() && l2.is_none() && carry == 0 {
            true => None,
            false => Some(Box::new(ListNode {
                next: Self::carried(
                    l1.and_then(|x| {
                        carry += x.val;
                        x.next
                    }),
                    l2.and_then(|x| {
                        carry += x.val;
                        x.next
                    }),
                    carry / 10,
                ),
                val: carry % 10,
            })),
        }
    }
}

pub fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(nums[0])));
    let mut p = head.as_mut();
    for num in nums.iter().skip(1) {
        let node = Some(Box::new(ListNode::new(*num)));
        p.as_mut().unwrap().next = node;
        p = p.unwrap().next.as_mut();
    }
    head
}

pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = vec![];
    let mut p = head;
    while let Some(node) = p {
        res.push(node.val);
        p = node.next;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 两个链表都为空
    #[test]
    #[ignore = "reason"]
    fn test_empty() {
        let l1 = None;
        let l2 = None;
        let expected = None;
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    /// 其中一个链表为空，另一个不为空
    #[test]
    #[ignore = "reason"]
    fn test_one_empty() {
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = None;
        let expected = create_list(vec![2, 4, 3]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    /// 两个链表长度相等
    #[test]
    fn test_same_length() {
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = create_list(vec![5, 6, 4]);
        let expected = create_list(vec![7, 0, 8]);
        assert_eq!(to_vec(Solution::add_two_numbers(l1, l2)), to_vec(expected));
    }

    /// 两个链表长度不相等
    #[test]
    #[ignore = "reason"]
    fn test_different_length() {
        let l1 = create_list(vec![9, 9, 9, 9, 9]);
        let l2 = create_list(vec![1]);
        let expected = create_list(vec![0, 0, 0, 0, 0, 1]);
        assert_eq!(to_vec(Solution::add_two_numbers(l1, l2)), to_vec(expected));
    }

    /// 两个链表相加进位
    #[test]
    #[ignore = "reason"]
    fn test_carry() {
        let l1 = create_list(vec![9, 9, 9, 9, 9]);
        let l2 = create_list(vec![9, 9, 9, 9, 9]);
        let expected = create_list(vec![8, 9, 9, 9, 9, 1]);
        assert_eq!(to_vec(Solution::add_two_numbers(l1, l2)), to_vec(expected));
    }
}
