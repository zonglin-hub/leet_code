//! 删除链表的倒数第 N 个结点
//!
//! 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。

use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    pub fn remove_nth_from_end_v1(head: ListNodePtr, n: i32) -> ListNodePtr {
        unsafe {
            let dummy = &mut ListNode { val: 0, next: head } as *mut ListNode;
            let mut slow = dummy;
            let mut fast = dummy;
            for _ in 0..n {
                fast = (*fast).next.as_mut().expect("").as_mut();
            }
            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().expect("").as_mut();
                slow = (*slow).next.as_mut().expect("").as_mut();
            }
            (*slow).next = (*slow).next.take().expect("").next;
            (*dummy).next.to_owned()
        }
    }
}

impl Solution {
    pub fn remove_nth_from_end_v2(head: ListNodePtr, n: i32) -> ListNodePtr {
        let (mut fast, mut slow) = (&head, &head);
        let mut root = ListNode::new(0);
        let mut curr = &mut root;
        let mut count = 0;

        while count < n {
            fast = &fast.as_ref().expect("").next;
            count += 1;
        }

        while fast.is_some() {
            fast = &fast.as_ref().expect("").next;

            let val = slow.as_ref().expect("").val;
            slow = &slow.as_ref().expect("").next;

            curr.next = Some(Box::new(ListNode::new(val)));
            curr = curr.next.as_mut().expect("");
        }

        curr.next = slow.as_ref().expect("").next.clone();

        root.next
    }
}
