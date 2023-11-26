//! 两数相加

use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    /// 递归
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
}

impl Solution {
    /// 双指针
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
            p = &mut p.as_mut().expect("").next;
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

    #[test]
    fn test_and_then() {
        fn sq_then_to_string(x: u32) -> Option<String> {
            x.checked_mul(x).map(|sq| sq.to_string())
        }

        assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));
        assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
        assert_eq!(None.and_then(sq_then_to_string), None);

        let arr_2d = [["A0", "A1"], ["B0", "B1"]];

        let item_0_1 = arr_2d.get(0).and_then(|row| row.get(1));
        assert_eq!(item_0_1, Some(&"A1"));

        let item_2_0 = arr_2d.get(2).and_then(|row| row.get(0));
        assert_eq!(item_2_0, None);
    }
}
