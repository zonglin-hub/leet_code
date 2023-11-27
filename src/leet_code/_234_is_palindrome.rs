//! 回文链表
//!

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn is_palindrome_v1(head: ListNodePtr) -> bool {
        let (mut val, mut node) = (vec![], &head);
        while let Some(node_box) = node {
            val.push(node_box.val);
            node = &node_box.next;
        }

        let val_rev = val.clone();
        val.reverse();
        val == val_rev
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{create_list, Solution};

    #[test]
    fn test_is_palindrome_v1() {
        assert_eq!(Solution::is_palindrome_v1(create_list(vec![1, 2])), false);
    }
}
