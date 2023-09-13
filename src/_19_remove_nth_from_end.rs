//! 删除链表的倒数第 N 个结点
//!
//! 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。

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

use crate::types::base_type::Solution;

impl Solution {
    pub fn remove_nth_from_end_v1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
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

    pub fn remove_nth_from_end_v2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (&head, &head);
        let mut root = ListNode::new(0);
        let mut curr = &mut root;
        let mut count = 0;

        while count < n {
            fast = &fast.as_ref().unwrap().next;
            count += 1;
        }

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

    fn create_tree_node(nums: i32) -> Option<Box<ListNode>> {
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

    #[test]
    #[ignore = "代码异常"]
    fn test_remove_nth_from_end_v1() {
        /*
            输入：head = [1,2,3,4,5], n = 2
            输出：[1,2,3,5]
        */
        assert_eq!(
            Solution::remove_nth_from_end_v1(create_list(vec![1, 2, 3, 4, 5]), 2),
            create_list(vec![1, 2, 3, 5])
        );

        /*
            输入：head = [1], n = 1
            输出：[]
        */
        assert_eq!(
            Solution::remove_nth_from_end_v1(create_list(vec![1]), 1),
            create_list(Vec::new())
        );

        /*
            输入：head = [1,2], n = 1
            输出：[1]
        */
        assert_eq!(
            Solution::remove_nth_from_end_v1(create_list(vec![1, 2]), 1),
            create_list(vec![1])
        );
    }

    #[test]
    #[ignore = "代码异常"]
    fn test_remove_nth_from_end_v2() {
        assert_eq!(
            Solution::remove_nth_from_end_v2(create_list(vec![1, 2, 3, 4, 5]), 2),
            create_list(vec![1, 2, 3, 5])
        );
        assert_eq!(
            Solution::remove_nth_from_end_v1(create_list(vec![1]), 1),
            create_list(Vec::new())
        );
        assert_eq!(
            Solution::remove_nth_from_end_v2(create_list(vec![1, 2]), 1),
            create_list(vec![1])
        );
    }
}
