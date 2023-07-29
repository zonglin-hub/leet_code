#![allow(unused)]
// Definition for singly-linked list.
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

pub struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
    ///
    /// 删除链表的倒数第 N 个结点
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        unsafe {
            let dummy = &mut ListNode { val: 0, next: head } as *mut ListNode;
            let mut slow = dummy;
            let mut fast = dummy;
            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap().as_mut();
            }
            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap().as_mut();
                slow = (*slow).next.as_mut().unwrap().as_mut();
            }
            (*slow).next = (*slow).next.take().unwrap().next;
            (*dummy).next.to_owned()
        }
    }

    pub fn remove_nth_from_end_1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (&head, &head);
        let mut root = ListNode::new(0);
        let mut curr = &mut root;
        let mut count = 0;

        while count < n {
            fast = &fast.as_ref().unwrap().next;
            count += 1;
        }

        // Find the K-th node which slow referred finally
        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;

            let val = slow.as_ref().unwrap().val;
            slow = &slow.as_ref().unwrap().next;

            curr.next = Some(Box::new(ListNode::new(val)));
            curr = curr.next.as_mut().unwrap();
        }

        curr.next = slow.as_ref().unwrap().next.clone();

        root.next
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

    fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = create_tree_node(nums[0]);
        let mut p = head.as_mut();
        for num in nums.iter().skip(1) {
            let node = create_tree_node(*num);
            p.as_mut().unwrap().next = node;
            p = p.unwrap().next.as_mut()
        }
        head
    }

    // fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    //     let mut head = Some(Box::new(ListNode::new(nums[0])));
    //     let mut p = head.as_mut();
    //     for num in nums.iter().skip(1) {
    //         let node = Some(Box::new(ListNode::new(*num)));
    //         p.as_mut().unwrap().next = node;
    //         p = p.unwrap().next.as_mut()
    //     }
    //     head
    // }

    #[test]
    fn test_remove_nth_from_end() {
        assert_eq!(
            Solution::remove_nth_from_end(create_list(vec![1, 2, 3, 4, 5]), 2),
            create_list(vec![1, 2, 3, 5])
        );
        assert_eq!(
            Solution::remove_nth_from_end(create_list(vec![1, 2]), 1),
            create_list(vec![1])
        );
    }
}
