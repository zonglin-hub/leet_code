//! 两数相加

use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    /// 模拟 (力扣官方题解)
    pub fn add_two_numbers(mut l1: ListNodePtr, mut l2: ListNodePtr) -> ListNodePtr {
        let mut head = None;
        let mut tail = &mut head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            if let Some(j1) = l1 {
                carry += j1.val;
                l1 = j1.next;
            }
            if let Some(j2) = l2 {
                carry += j2.val;
                l2 = j2.next;
            }
            *tail = Some(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
            tail = &mut tail.as_mut()?.next;
        }
        head
    }

    /// 递归
    ///
    /// 用于将两个链表的元素相加并返回结果。
    /// 其中`carried`函数用于处理进位，并返回结果。在` carried `函数中，我们使用` match `语句检查是否存在进位。
    /// 如果没有进位，我们返回` None `表示没有结果；
    /// 否则，我们创建一个新的链表节点，并将进位加到当前节点的值上。
    /// 然后，我们递归地调用` carried `函数，处理下一个节点，并将进位除以 10 作为下一次递归的进位。
    /// 最后，我们将所有的节点连接起来，返回结果。
    pub fn add_two_numbers_v1(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
        #[inline]
        fn carried(l1: ListNodePtr, l2: ListNodePtr, mut carry: i32) -> ListNodePtr {
            match l1.is_none() && l2.is_none() && carry == 0 {
                true => None,
                false => Some(Box::new(ListNode {
                    next: carried(
                        l1.and_then(|x| {
                            carry += x.val;
                            x.next
                        }),
                        l2.and_then(|x| {
                            carry += x.val;
                            x.next
                        }),
                        carry / 10,
                    ),
                    val: carry % 10,
                })),
            }
        }
        carried(l1, l2, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(Solution::add_two_numbers(None, None), None);
        assert_eq!(
            Solution::add_two_numbers(linked_list!(2, 4, 3), linked_list!(5, 6, 4)),
            linked_list!(7, 0, 8)
        );
        assert_eq!(
            Solution::add_two_numbers(linked_list!(9, 9, 9, 9, 9, 9, 9), linked_list!(9, 9, 9, 9)),
            linked_list!(8, 9, 9, 9, 0, 0, 0, 1)
        );
    }

    #[test]
    fn test_add_two_numbers_v1() {
        assert_eq!(Solution::add_two_numbers_v1(None, None), None);
        assert_eq!(
            Solution::add_two_numbers_v1(linked_list!(2, 4, 3), linked_list!(5, 6, 4)),
            linked_list!(7, 0, 8)
        );
        assert_eq!(
            Solution::add_two_numbers_v1(
                linked_list!(9, 9, 9, 9, 9, 9, 9),
                linked_list!(9, 9, 9, 9)
            ),
            linked_list!(8, 9, 9, 9, 0, 0, 0, 1)
        );
    }
}
