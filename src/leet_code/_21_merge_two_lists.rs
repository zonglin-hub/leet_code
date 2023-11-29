//! 合并两个有序链表
//!
//! 输入两个递增排序的链表，合并这两个链表并使新链表中的节点仍然是递增排序的。

use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    /// 这个函数的作用是将两个有序链表合并成一个新的有序链表。它接受两个链表的头节点指针作为输入，并返回合并后的链表头节点指针。
    ///
    /// 首先，函数定义了一个名为`carried`的内部函数，用于处理两个链表的合并操作。在`carried`函数中，它首先检查两个链表是否都为空。
    /// 如果都为空，那么返回`None`表示没有结果。然后，它使用模式匹配来处理不同的情况。
    ///
    /// 在模式匹配中，它首先检查第一个链表是否为空。如果为空，那么直接返回第二个链表。
    /// 然后，它检查第二个链表是否为空。如果为空，那么直接返回第一个链表。
    /// 最后，它检查两个链表都不为空的情况，并根据第一个链表的当前节点的值是否小于等于第二个链表的当前节点的值来决定如何合并。
    ///
    /// 如果第一个链表的当前节点的值小于等于第二个链表的当前节点的值，那么将第一个链表的当前节点作为新的链表头节点，并将第二个链表的当前节点作为新的链表的下一个节点。
    /// 然后，将新的链表头节点和新的链表的下一个节点作为参数调用`carried`函数，以继续合并剩余的节点。最后，返回新的链表头节点。
    ///
    /// 如果第一个链表的当前节点的值大于第二个链表的当前节点的值，那么将第二个链表的当前节点作为新的链表头节点，并将第一个链表的当前节点作为新的链表的下一个节点。
    /// 然后，将新的链表头节点和新的链表的下一个节点作为参数调用`carried`函数，以继续合并剩余的节点。最后，返回新的链表头节点。
    ///
    /// 在`merge_two_lists_v1`函数中，它首先调用`carried`函数来合并两个链表，并返回合并后的链表头节点指针。
    pub fn merge_two_lists(list1: ListNodePtr, list2: ListNodePtr) -> ListNodePtr {
        fn carried(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
            if l1.is_none() && l2.is_none() {
                return None;
            }

            match (l1, l2) {
                (None, None) => None,
                (None, r) => r,
                (l, None) => l,
                (Some(mut l), Some(mut r)) => match l.val <= r.val {
                    true => {
                        l.next = carried(l.next, Some(r));
                        Some(l)
                    }
                    false => {
                        r.next = carried(Some(l), r.next);
                        Some(r)
                    }
                },
            }
        }

        carried(list1, list2)
    }

    /// 这段代码实现了一个合并两个有序链表的函数`merge_two_lists`。它接受两个链表的头节点指针作为输入，并返回合并后的链表头节点指针。
    ///
    /// 首先，它使用模式匹配来处理不同的情况。如果其中一个链表为空，那么直接返回另一个链表。
    /// 如果两个链表都为空，那么返回`None`表示没有结果。如果两个链表都不为空，那么它比较两个链表的当前节点的值。
    /// 如果第一个链表的当前节点的值大于等于第二个链表的当前节点的值，那么将第二个链表的当前节点作为新的链表头节点，并将第一个链表的当前节点作为新的链表的下一个节点。
    /// 然后，将新的链表头节点和新的链表的下一个节点作为参数调用`merge_two_lists`函数，以继续合并剩余的节点。最后，返回新的链表头节点。
    ///
    /// 如果第一个链表的当前节点的值小于第二个链表的当前节点的值，那么将第一个链表的当前节点作为新的链表头节点，并将第二个链表的当前节点作为新的链表的下一个节点。
    /// 然后，将新的链表头节点和新的链表的下一个节点作为参数调用`merge_two_lists`函数，以继续合并剩余的节点。最后，返回新的链表头节点。
    ///
    /// 在`merge_two_lists`函数中，它使用模式匹配来处理不同的情况，并使用`Box::new`函数来创建新的链表节点。
    pub fn _merge_two_lists(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
        match (l1, l2) {
            (Some(n), None) | (None, Some(n)) => Some(n),
            (None, None) => None,
            (Some(l1), Some(l2)) => match l1.val >= l2.val {
                true => Some(Box::new(ListNode {
                    val: l2.val,
                    next: Self::merge_two_lists(Some(l1), l2.next),
                })),
                false => Some(Box::new(ListNode {
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2)),
                })),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
        assert_eq!(
            Solution::merge_two_lists(linked_list!(1, 2, 4), linked_list!(1, 3, 4)),
            linked_list!(1, 1, 2, 3, 4, 4)
        );
        assert_eq!(
            Solution::merge_two_lists(None, linked_list!(0)),
            linked_list!(0)
        );
    }
}
