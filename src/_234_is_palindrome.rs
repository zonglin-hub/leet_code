//! 回文链表
//!

use crate::{ListNode, Solution};

impl Solution {
    pub fn is_palindrome_v1(head: Option<Box<ListNode>>) -> bool {
        let (mut val, mut node) = (vec![], &head);
        // loop {
        //     let node_box = match node {
        //         Some(x) => x,
        //         None => break,
        //     };
        //     val.push(node_box.val);
        //     node = &node_box.next;
        // }

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

    use crate::create_list;

    use super::*;

    #[test]
    fn test_is_palindrome_v1() {
        assert_eq!(Solution::is_palindrome_v1(create_list(vec![1, 2])), false);
    }
}
