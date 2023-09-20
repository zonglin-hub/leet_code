//! 两数相加

use super::{ListNode, Solution};

impl Solution {
    /// 递归
    pub fn add_two_numbers_v1(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::carried(l1, l2, 0)
    }

    fn carried(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        mut carry: i32,
    ) -> Option<Box<ListNode>> {
        // if l1.is_none() && l2.is_none() && carry == 0 {
        //     None
        // } else {
        //     Some(Box::new(ListNode {
        //         next: carried(
        //             l1.and_then(|x| {carry += x.val; x.next}),
        //             l2.and_then(|x| {carry += x.val; x.next}),
        //             carry / 10
        //         ),
        //         val: carry % 10
        //     }))
        // }
        // 首先，我们检查l1和l2是否都为None，并且carry是否为0。
        match l1.is_none() && l2.is_none() && carry == 0 {
            // 如果是，那么如果链表为空，返回None；
            true => None,
            false => Some(Box::new(ListNode {
                // 如果carry为0，则将l1和l2的值相加，并将结果放入到l1的next中，将carry减1
                next: Self::carried(
                    l1.and_then(|x| {
                        carry += x.val;
                        x.next
                    }),
                    l2.and_then(|x| {
                        carry += x.val;
                        x.next
                    }),
                    // 并将carry / 10赋值给next指针。
                    carry / 10,
                ),
                // 否则，创建一个新的节点，其值为carry % 10，
                val: carry % 10,
            })),
        }
    }
}

impl Solution {
    /// 双指针
    pub fn add_two_numbers_v2(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 创建一个新的链表
        let mut new_list = None;
        let mut p = &mut new_list;
        // 用于表示双数 进位
        let mut carry = 0;

        // while l1!= None || l2!= None || t!= 0 {
        while l1.is_some() || l2.is_some() || carry != 0 {
            // 如果l1不为空，则将l1的值加到carry上
            if let Some(j1) = l1 {
                carry += j1.val;
                l1 = j1.next;
            }
            // 如果l2不为空，则将l2的值加到carry上
            if let Some(j2) = l2 {
                carry += j2.val;
                l2 = j2.next;
            }
            // 将carry的值转换为十进制，并将其转换为十六进制，并将其加到p中
            *p = Some(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
            p = &mut p.as_mut().expect("").next;
        }
        new_list
    }
}
