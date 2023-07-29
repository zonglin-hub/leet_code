#![allow(unused)]
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
    /// https://leetcode.cn/problems/merge-two-sorted-lists/
    ///
    /// 合并两个有序链表
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::carried(list1, list2)
    }

    fn carried(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => match l.val <= r.val {
                true => {
                    l.next = Self::carried(l.next, Some(r));
                    Some(l)
                }
                false => {
                    r.next = Self::carried(Some(l), r.next);
                    Some(r)
                }
            },
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

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(
            Solution::merge_two_lists(create_list(vec![1, 2, 4]), create_list(vec![1, 3, 4])),
            create_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}
