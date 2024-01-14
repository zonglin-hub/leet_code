//! 奇偶链表

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn odd_even_list(head: ListNodePtr) -> ListNodePtr {
        let mut odd_head = head?;
        let mut odd = odd_head.as_mut();
        if let Some(mut node) = odd.next.take() {
            let mut even = node.as_mut();
            while even.next.is_some() {
                odd.next = even.next.take();
                odd = odd.next.as_mut()?;
                even.next = odd.next.take();
                if even.next.is_some() {
                    even = even.next.as_mut()?;
                }
            }
            odd.next = Some(node);
        }
        Some(odd_head)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_odd_even_list() {
        assert_eq!(
            Solution::odd_even_list(linked_list!(1, 2, 3, 4, 5)),
            linked_list!(1, 3, 5, 2, 4)
        );
        assert_eq!(
            Solution::odd_even_list(linked_list!(2, 1, 3, 5, 6, 4, 7)),
            linked_list!(2, 3, 6, 7, 1, 5, 4)
        );
        assert_eq!(Solution::odd_even_list(linked_list!(1)), linked_list!(1));
        assert_eq!(Solution::odd_even_list(None), None);
    }
}
