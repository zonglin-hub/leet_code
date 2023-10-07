//! 两数相加

use super::{ListNode, ListNodePtr, Solution};

/// 整数相加
impl Solution {
    /// 递归
    pub fn add_two_numbers_2_v1(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
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

    /// 双指针
    pub fn add_two_numbers_2_v2(mut l1: ListNodePtr, mut l2: ListNodePtr) -> ListNodePtr {
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
            p = &mut p.as_mut().expect("").next;
        }
        new_list
    }
}
