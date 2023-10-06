//! 两数相加 2 | 415

use super::{to_int_vec, ListNode, ListNodePtr, Solution};
use std::iter::repeat;

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

/// 字符串相加
impl Solution {
    pub fn add_strings_415_v1(nums1: String, nums2: String) -> String {
        let s1 = to_int_vec(&nums1);
        let s2 = to_int_vec(&nums2);
        let mut carry = 0;
        let mut s3 = vec![];
        let n1 = s1.len();
        let n2 = s2.len();
        let mut i = 0;

        while i < n1 || i < n2 || carry > 0 {
            let mut v = 0;

            if i < n1 {
                v += s1[i];
            }

            if i < n2 {
                v += s2[i];
            }

            v += carry;
            carry = v / 10;
            s3.push(((v % 10) as u8 + b'0') as char);
            i += 1;
        }

        s3.iter().rev().collect()
    }

    pub fn add_strings(num1: String, num2: String) -> String {
        if num2.len() > num1.len() {
            return Self::add_strings(num2, num1);
        }
        let mut prev = 0;
        let mut ret = num1
            .chars()
            .rev()
            .zip(
                num2.chars()
                    .rev()
                    .chain(repeat('0').take(num1.len().saturating_sub(num2.len()))),
            )
            .map(|(a, b)| {
                let curr = prev + a.to_digit(10).expect("") + b.to_digit(10).expect("");
                prev = curr / 10;
                char::from_digit(curr % 10, 10).expect("")
            })
            .collect::<Vec<_>>();

        if prev == 1 {
            ret.push((1u8 + b'0') as char);
        }
        ret.iter().rev().collect::<_>()
    }
}
