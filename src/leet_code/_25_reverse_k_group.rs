//! K 个一组翻转链表

use crate::{ListNode, Solution};

impl Solution {
    pub fn reverse_k_group_v1(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut pre: Option<Box<ListNode>> = None;
            let mut head = head;
            let mut next;

            while head.is_some() {
                next = head.as_mut().expect("").next.take();
                head.as_mut().expect("").next = pre.take();
                pre = head.take();
                head = next.take()
            }

            pre
        }

        if k == 1 {
            return head;
        }

        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().expect("").next = head;
        let mut pre = &mut dummy as *mut Option<Box<ListNode>>;
        let mut end = pre;
        // let mut end = &mut dummy as *mut Option<Box<ListNode>>;

        unsafe {
            while end.as_ref().expect("").as_ref().expect("").next.is_some() {
                for _ in 0..k {
                    if end.as_ref().expect("").is_none() {
                        break;
                    }
                    if !end.is_null() {
                        end = &mut end.as_mut().expect("").as_mut().expect("").next;
                    }
                }

                if end.as_ref().expect("").is_none() {
                    break;
                }

                let mut start = pre.as_mut().expect("").as_mut().expect("").next.take();
                pre = &mut dummy;

                while pre.as_ref().expect("").as_ref().expect("").next.is_some() {
                    pre = &mut pre.as_mut().expect("").as_mut().expect("").next;
                }

                let mut next = end.as_mut().expect("").as_mut().expect("").next.take();
                let startp = &mut start as *mut Option<Box<ListNode>>;
                end.as_mut().expect("").as_mut().expect("").next = None;
                pre.as_mut().expect("").as_mut().expect("").next = reverse(start).take();
                startp.as_mut().expect("").as_mut().expect("").next = next.take();
                pre = startp;
                end = pre;
            }
        }
        dummy.as_mut().expect("").next.take()
    }
}
