//! 移除链表元素
//!
//! 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn remove_elements(head: ListNodePtr, val: i32) -> ListNodePtr {
        match head {
            None => None,
            Some(mut node) => match node.val == val {
                true => Self::remove_elements(node.next, val),
                false => {
                    node.next = Self::remove_elements(node.next, val);
                    Some(node)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{create_list, Solution};

    #[test]
    fn test_remove_elements() {
        assert_eq!(
            Solution::remove_elements(create_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
            create_list(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(Solution::remove_elements(None, 1), None);
        assert_eq!(
            Solution::remove_elements(create_list(vec![7, 7, 7, 7]), 7),
            None
        );
    }
}
