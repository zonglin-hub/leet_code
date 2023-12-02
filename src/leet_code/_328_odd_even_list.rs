//! 奇偶链表

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn odd_even_list(head: ListNodePtr) -> ListNodePtr {
        if head.is_none() {
            return head;
        }

        let mut odd_head = head?;
        let mut even_head = odd_head.next.take()?;
        let (mut odd, mut even) = (&mut odd_head, &mut even_head);

        while even.next.is_some() {
            odd.next = even.next.take();
            odd = odd.next.as_mut()?;

            if odd.next.is_none() {
                break;
            }

            even.next = odd.next.take();
            even = even.next.as_mut()?;
        }

        odd.next = Some(even_head);
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
    }
}
