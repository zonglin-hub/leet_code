#![allow(unused)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    /// 移除链表元素
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
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
