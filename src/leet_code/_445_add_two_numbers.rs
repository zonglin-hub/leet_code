use crate::leet_code::ListNode;

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn add_two_numbers_445(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
        fn carried(l1: ListNodePtr, l2: ListNodePtr, mut carry: i32) -> ListNodePtr {
            match l1.is_none() && l2.is_none() && carry == 0 {
                true => None,
                false => Some(Box::new(ListNode {
                    next: carried(
                        l1.and_then(|x| {
                            carry += x.val;
                            x.next
                        }),
                        l2.and_then(|x| {
                            carry += x.val;
                            x.next
                        }),
                        carry / 10,
                    ),
                    val: carry % 10,
                })),
            }
        }

        fn reverse_list(mut head: ListNodePtr) -> ListNodePtr {
            let mut ptr = None;
            while let Some(mut node) = head {
                head = node.next.take();
                node.next = ptr;
                ptr = Some(node);
            }
            ptr
        }

        reverse_list(carried(reverse_list(l1), reverse_list(l2), 0))
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_add_two_numbers_445() {
        assert_eq!(
            Solution::add_two_numbers_445(linked_list!(7, 2, 4, 3), linked_list!(5, 6, 4)),
            linked_list!(7, 8, 0, 7)
        );
        assert_eq!(
            Solution::add_two_numbers_445(linked_list!(2, 4, 3), linked_list!(5, 6, 4)),
            linked_list!(8, 0, 7)
        );
        assert_eq!(
            Solution::add_two_numbers_445(linked_list!(0), linked_list!(0)),
            linked_list!(0)
        );
    }
}
