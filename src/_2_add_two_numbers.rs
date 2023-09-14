//! 两数相加
//!
//! 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。

use crate::types::base_type::{ListNode, Solution};

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

#[cfg(test)]
mod tests {
    use super::*;

    pub fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        // 创建一个头节点，并将其赋值给head
        let mut head = Some(Box::new(ListNode::new(nums[0])));
        // 将head赋值给p
        let mut p = head.as_mut();
        // 遍历nums数组，将每一个元素赋值给ListNode
        for num in nums.iter().skip(1) {
            let node = Some(Box::new(ListNode::new(*num)));
            // 将ListNode赋值给p的下一个节点
            p.as_mut().expect("").next = node;
            // 将p的下一个节点赋值给p
            p = p.expect("").next.as_mut();
        }
        // 返回head
        head
    }

    pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        // 创建一个空的数组，用于存放节点的值
        let mut res = vec![];
        // 将head赋值给p
        let mut p = head;
        // 当p存在时，将其值存入res数组
        while let Some(node) = p {
            res.push(node.val);
            // 将p的下一个节点赋值给p
            p = node.next;
        }
        // 返回res数组
        res
    }

    #[test]
    fn test_add_two_numbers_v1() {
        /*
            输入：l1 = [0], l2 = [0]
            输出：[0]
        */
        let l1 = None;
        let l2 = None;
        let expected = None;
        assert_eq!(Solution::add_two_numbers_v1(l1, l2), expected);

        /*
            输入：l1 = [2,4,3], l2 = [5,6,4]
            输出：[7,0,8]
            解释：342 + 465 = 807.
        */
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = create_list(vec![5, 6, 4]);
        let expected = create_list(vec![7, 0, 8]);
        assert_eq!(
            to_vec(Solution::add_two_numbers_v1(l1, l2)),
            to_vec(expected)
        );

        /*
            输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
            输出：[8,9,9,9,0,0,0,1]
        */
        let l1 = create_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_list(vec![9, 9, 9, 9]);
        let expected = create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(
            to_vec(Solution::add_two_numbers_v1(l1, l2)),
            to_vec(expected)
        );
    }

    #[test]
    fn test_add_two_numbers_v2() {
        let l1 = None;
        let l2 = None;
        let expected = None;
        assert_eq!(Solution::add_two_numbers_v2(l1, l2), expected);

        let l1 = create_list(vec![2, 4, 3]);
        let l2 = create_list(vec![5, 6, 4]);
        let expected = create_list(vec![7, 0, 8]);
        assert_eq!(
            to_vec(Solution::add_two_numbers_v2(l1, l2)),
            to_vec(expected)
        );

        let l1 = create_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_list(vec![9, 9, 9, 9]);
        let expected = create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(
            to_vec(Solution::add_two_numbers_v2(l1, l2)),
            to_vec(expected)
        );
    }
}
