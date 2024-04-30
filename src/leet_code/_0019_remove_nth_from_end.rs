//! 删除链表的倒数第 N 个结点
//!
//! 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。

use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    /// 该函数的实现思路是，创建一个指向头节点的指针`dummy`，并将其初始化为`head`。
    /// 然后，使用两个指针`slow`和`fast`来遍历链表。在遍历过程中，`fast`指针比`slow`指针快`n`步。
    /// 当`fast`指针到达链表的末尾时，`slow`指针指向的节点就是要删除的节点。
    /// 然后，将`slow`指针指向的节点的下一个节点设置为`slow`指针的下一个节点的下一个节点。
    /// 最后，返回头节点的下一个节点，即删除了指定节点后的链表。
    ///
    /// 需要注意的是，该函数使用了`unsafe`代码，因为它直接操作了指针。在实际使用中，应该仔细考虑是否需要使用`unsafe`代码，并确保代码的安全性。
    #[allow(unknown_lints)]
    pub fn remove_nth_from_end_unsafe(head: ListNodePtr, n: i32) -> ListNodePtr {
        unsafe {
            let dummy = &mut ListNode { val: 0, next: head } as *mut ListNode;
            let mut slow = dummy;
            let mut fast = dummy;
            for _ in 0..n {
                fast = (*fast).next.as_mut()?.as_mut();
            }

            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut()?.as_mut();
                slow = (*slow).next.as_mut()?.as_mut();
            }
            (*slow).next = (*slow).next.take()?.next;
            (*dummy).next.to_owned()
        }
    }

    pub fn remove_nth_from_end_safe(head: ListNodePtr, n: i32) -> ListNodePtr {
        let mut root = ListNode::new(0);
        let mut fast = &head;
        let mut slow = &head;

        let mut curr = &mut root;

        let mut count = 0;
        while count < n {
            fast = &fast.as_ref()?.next;
            count += 1;
        }

        while fast.is_some() {
            fast = &fast.as_ref()?.next;
            curr.next = Some(Box::new(ListNode::new(slow.as_ref()?.val)));
            slow = &slow.as_ref()?.next;
            curr = curr.next.as_mut()?;
        }

        // curr.next = slow.as_ref()?.next.clone();
        curr.next.clone_from(&slow.as_ref()?.next);
        root.next
    }

    // pub fn remove_nth_from_end_scan_deep(head: ListNodePtr, n: i32) -> ListNodePtr {
    //     fn scan_deep(deep: i32, node: &mut ListNodePtr, n: i32) -> i32 {
    //         if node.is_none() {
    //             return deep;
    //         }
    //         let deepest = scan_deep(deep + 1, &mut node.as_mut().unwrap().next, n);
    //         if deepest - n == deep {
    //             // *node = node.as_ref().unwrap().next.clone()
    //             // node 被借用，无法使用clone_from
    //             node.clone_from(&node.as_ref().unwrap().next)
    //         }
    //         deepest
    //     }
    //     let mut head = head;
    //     scan_deep(0, &mut head, n);
    //     head
    // }
}

#[cfg(test)]
mod tests {
    use crate::{
        leet_code::{ListNode, Solution},
        linked_list,
    };

    #[test]
    fn test_remove_nth_from_end_unsafe() {
        assert_eq!(Solution::remove_nth_from_end_unsafe(linked_list!(1), 1), None);
        assert_eq!(Solution::remove_nth_from_end_unsafe(linked_list!(1, 2), 1), linked_list!(1));
        assert_eq!(
            Solution::remove_nth_from_end_unsafe(linked_list!(1, 2, 3, 4, 5), 2),
            linked_list!(1, 2, 3, 5)
        );
    }

    #[test]
    fn test_remove_nth_from_end_safe() {
        assert_eq!(Solution::remove_nth_from_end_safe(linked_list!(1), 1), None);
        assert_eq!(Solution::remove_nth_from_end_safe(linked_list!(1, 2), 1), linked_list!(1));
        assert_eq!(
            Solution::remove_nth_from_end_safe(linked_list!(1, 2, 3, 4, 5), 2),
            linked_list!(1, 2, 3, 5)
        );
    }

    // #[test]
    // fn test_remove_nth_from_end_scan_deep() {
    //     assert_eq!(Solution::remove_nth_from_end_scan_deep(linked_list!(1), 1), None);
    //     assert_eq!(Solution::remove_nth_from_end_scan_deep(linked_list!(1, 2), 1), linked_list!(1));
    //     assert_eq!(
    //         Solution::remove_nth_from_end_scan_deep(linked_list!(1, 2, 3, 4, 5), 2),
    //         linked_list!(1, 2, 3, 5)
    //     );
    // }
}
