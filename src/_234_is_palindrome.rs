use crate::types::base_type::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    /// 回文链表
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
