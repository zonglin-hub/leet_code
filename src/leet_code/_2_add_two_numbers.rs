//! 两数相加

use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    /// 用于将两个链表的元素相加并返回结果。
    /// 其中，`carried`函数用于处理进位，并返回结果。在` carried `函数中，我们使用` match `语句检查是否存在进位。
    /// 如果没有进位，我们返回` None `表示没有结果；否则，我们创建一个新的链表节点，并将进位加到当前节点的值上。
    /// 然后，我们递归地调用` carried `函数，处理下一个节点，并将进位除以 10 作为下一次递归的进位。
    /// 最后，我们将所有的节点连接起来，返回结果。
    pub fn add_two_numbers(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
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

    /// 这个函数的实现思路是将两个链表的元素相加，然后将结果构建成一个新的链表。
    /// 具体来说，函数首先创建一个空的链表`new_list`，并将其初始化为`None`。然后，它使用一个循环来处理两个链表中的元素。
    /// 在每次循环中，它首先检查两个链表中是否有元素。如果有，它将这些元素的值加到变量`carry`中，并将链表的下一个节点设置为下一个元素。
    /// 然后，它将`carry`除以 10，并将余数作为新的链表节点的值添加到`new_list`中。
    /// 最后，它将`new_list`返回。
    pub fn _add_two_numbers(mut l1: ListNodePtr, mut l2: ListNodePtr) -> ListNodePtr {
        let mut new_list = None;
        let mut p = &mut new_list;
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

            *p = Some(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
            p = &mut p.as_mut().unwrap().next;
        }
        new_list
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{create_list, to_vec, ListNode, Solution};

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(Solution::add_two_numbers(None, None), None);
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None }))
                }))
            }))
        );
        assert_eq!(
            to_vec(Solution::add_two_numbers(
                create_list(vec![9, 9, 9, 9, 9, 9, 9]),
                create_list(vec![9, 9, 9, 9])
            )),
            to_vec(create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]))
        );
    }
}
