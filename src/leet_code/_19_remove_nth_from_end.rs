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
    pub fn remove_nth_from_end(head: ListNodePtr, n: i32) -> ListNodePtr {
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

    /// 这段代码的实现思路。这段代码的作用是从一个链表的末尾删除指定的第`n`个元素，并返回新的链表头指针。
    ///
    /// 首先，我们创建了两个指针`fast`和`slow`，并将它们初始化为链表的头节点。然后，我们使用一个循环，让`fast`指针向前移动`n`步，以便在删除元素时能够正确定位到要删除的元素。
    ///
    /// 接下来，我们使用另一个循环，让`fast`指针和`slow`指针同时向前移动，直到`fast`指针指向链表的末尾。在每次循环中，我们将`slow`指针指向的元素的值添加到新的链表中，并将`slow`指针向前移动一步。
    ///
    /// 当`fast`指针指向链表的末尾时，我们将`slow`指针指向的元素的下一个节点设置为新的链表头指针，并返回新的链表头指针。
    ///
    /// 需要注意的是，这段代码使用了`unsafe`代码，因为它直接操作了指针。在实际使用中，应该仔细考虑是否需要使用`unsafe`代码，并确保代码的安全性。
    pub fn _remove_nth_from_end(head: ListNodePtr, n: i32) -> ListNodePtr {
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
    use crate::leet_code::{create_list, Solution};

    #[test]
    fn test_remove_nth_from_end() {
        assert_eq!(
            Solution::remove_nth_from_end(create_list(vec![1, 2, 3, 4, 5]), 2),
            create_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(create_list(vec![1]), 1), None);
        assert_eq!(
            Solution::remove_nth_from_end(create_list(vec![1, 2]), 1),
            create_list(vec![1])
        );
    }
}
