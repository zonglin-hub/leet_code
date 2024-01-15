use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    /// 这段代码实现了一个将链表中指定范围内的节点进行反转的函数`reverse_between`。它接受一个链表的头节点指针、反转范围的左边界和右边界作为输入，并返回反转后的链表头节点指针。
    ///
    /// 首先，它创建了一个哑元节点，并将其设置为链表的头节点。
    /// 然后，它使用一个循环来遍历链表，直到到达反转范围的左边界。
    /// 在每次循环中，它将哑元节点的下一个节点设置为当前节点，并将当前节点的下一个节点设置为哑元节点的下一个节点。
    /// 这样，哑元节点就成为了反转范围的左边界。
    ///
    /// 接下来，它使用一个循环来遍历反转范围内的节点。
    /// 在每次循环中，它将当前节点的下一个节点取走，并将其作为下一个节点的下一个节点。
    /// 然后，它将当前节点的下一个节点设置为哑元节点的下一个节点，并将哑元节点的下一个节点设置为当前节点的下一个节点。
    /// 这样，反转范围内的节点就被反转了。
    ///
    /// 最后，它使用一个循环来遍历哑元节点的下一个节点，直到到达反转范围的右边界。
    /// 在每次循环中，它将哑元节点的下一个节点设置为当前节点，并将当前节点的下一个节点设置为哑元节点的下一个节点。
    /// 这样，哑元节点就成为了反转范围的右边界。
    ///
    /// 最后，它返回哑元节点的下一个节点，即反转后的链表头节点指针。
    pub fn reverse_between(head: ListNodePtr, left: i32, right: i32) -> ListNodePtr {
        let mut dummy_node = Some(Box::new(ListNode { val: -1, next: head }));

        let mut pre = &mut dummy_node;
        for _ in 0..left - 1 {
            pre = &mut pre.as_mut()?.next;
        }

        let mut cur = pre.as_mut()?.next.take();
        let mut next;

        for _ in 0..(right - left) {
            next = cur.as_mut()?.next.take();
            cur.as_mut()?.next = next.as_mut()?.next.take();
            next.as_mut()?.next = pre.as_mut()?.next.take();
            pre.as_mut()?.next = next.take();
        }

        while pre.as_mut()?.next.is_some() {
            pre = &mut pre.as_mut()?.next;
        }
        pre.as_mut()?.next = cur.take();
        dummy_node?.next
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        leet_code::{ListNode, Solution},
        linked_list,
    };

    #[test]
    fn test_reverse_between() {
        assert_eq!(
            Solution::reverse_between(linked_list!(1, 2, 3, 4, 5), 2, 4),
            linked_list!(1, 4, 3, 2, 5)
        );
        assert_eq!(Solution::reverse_between(linked_list!(5), 1, 1), linked_list!(5));
    }
}
