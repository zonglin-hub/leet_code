//! K 个一组翻转链表

use super::{ListNodePtr, Solution};

impl Solution {
    /// 这段代码实现了一个将链表的每 k 个元素反转的函数`reverse_k_group`。它接受一个链表的头节点指针和一个整数 k 作为输入，并返回反转后的链表头节点指针。
    ///
    /// 首先，它使用一个循环来移动头节点指针，以便将链表的第一个 k 个元素移动到链表的末尾。
    /// 在每次循环中，它检查头节点指针是否为空。如果为空，那么表示已经移动了 k 个元素，因此可以返回头节点指针。
    /// 如果头节点指针不为空，那么它将头节点指针向后移动一步，并将下一个节点指针设置为`next_head`。
    ///
    /// 然后，它使用一个内部函数`reverse_k_group`来反转链表的下一个 k 个元素，并将结果存储在`new_head`变量中。
    /// 在每次循环中，它将头节点指针向前移动一步，并将下一个节点指针设置为`new_head`。最后，它将`new_head`作为结果返回。
    ///
    /// 在`reverse_k_group`函数中，它使用模式匹配来处理不同的情况，并使用`Box::new`函数来创建新的链表节点。
    pub fn reverse_k_group(mut head: ListNodePtr, k: i32) -> ListNodePtr {
        let mut next_head = &mut head;
        for _ in 0..k {
            if let Some(node) = next_head.as_mut() {
                next_head = &mut node.next;
            } else {
                return head;
            }
        }

        let mut new_head = Self::reverse_k_group(next_head.take(), k);
        for _ in 0..k {
            if let Some(mut node) = head {
                head = node.next.take();
                node.next = new_head.take();
                new_head = Some(node);
            }
        }
        new_head
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        leet_code::{ListNode, Solution},
        linked_list,
    };

    #[test]
    fn test_reverse_k_group() {
        assert_eq!(
            Solution::reverse_k_group(linked_list!(1, 2, 3, 4, 5), 2),
            linked_list!(2, 1, 4, 3, 5)
        );
        assert_eq!(
            Solution::reverse_k_group(linked_list!(1, 2, 3, 4, 5), 3),
            linked_list!(3, 2, 1, 4, 5)
        );
    }
}
