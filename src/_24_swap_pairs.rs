//! 两两交换链表中的节点

#![allow(unused)]

use crate::{ListNode, Solution};

impl Solution {
    /// 这道题的解题思路是使用递归。
    ///
    /// 我们首先判断链表是否为空或只有一个节点，如果是，则直接返回该节点。
    /// 否则，我们将链表分为两个部分：头节点和其余部分。该问题的子问题是反转剩余部分的链表。
    /// 我们对剩余部分进行递归调用，然后将头节点与逆转的子链表合并。
    /// 最后返回合并后的链表。在合并时，我们需要将原来的头节点连接到已经反转的子链表的尾部。
    pub fn swap_pairs_v1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // head.and_then(|mut n| match n.next {
        //     None => None,
        //     Some(mut m) => {
        //         n.next = Self::swap_pairs_v1(m.next);
        //         m.next = Some(n);
        //         Some(m)
        //     }
        // })
        // head.and_then(|mut n| match n.next {
        //     None => Some(n), // 添加这一行以确保奇数长度的链表最后一个节点被正确返回
        //     Some(mut m) => {
        //         n.next = Self::swap_pairs_v1(m.next);
        //         m.next = Some(n);
        //         Some(m)
        //     }
        // })
        head.map(|mut n| match n.next {
            None => n, // 添加这一行以确保奇数长度的链表最后一个节点被正确返回
            Some(mut m) => {
                n.next = Self::swap_pairs_v1(m.next);
                m.next = Some(n);
                m
            }
        })
    }
}
