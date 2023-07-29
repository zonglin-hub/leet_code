#![allow(unused)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut n| match n.next {
            None => None,
            Some(mut m) => {
                n.next = Self::swap_pairs(m.next);
                m.next = Some(n);
                Some(m)
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_linked_list(values: &Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;

        for &val in values.iter().rev() {
            let node = ListNode {
                val,
                next: head.take(),
            };

            head = Some(Box::new(node));
        }

        head
    }

    #[test]
    fn test_swap_pairs() {
        assert_eq!(
            Solution::swap_pairs(create_linked_list(&vec![1, 2, 3, 4])),
            create_linked_list(&vec![2, 1, 4, 3])
        );
    }
}
