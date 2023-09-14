//! 剑指 Offer 06. 从尾到头打印链表
//!
//! 输入一个链表的头节点，从尾到头反过来返回每个节点的值（用数组返回）。

use crate::{ListNode, Solution};

impl Solution {
    /// 从尾到头打印链表
    pub fn reverse_print_v1(head: Option<Box<ListNode>>) -> Vec<i32> {
        let (mut res, mut node) = (vec![], &head);
        while let Some(x) = node {
            res.push(x.val);
            node = &x.next;
        }
        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::create_list;

    use super::*;

    #[test]
    fn test_reverse_print_v1() {
        /*
            输入：head = [1,3,2]
            输出：[2,3,1]
        */
        assert_eq!(
            Solution::reverse_print_v1(create_list(vec![1, 3, 2])),
            vec![2, 3, 1]
        );
    }
}
