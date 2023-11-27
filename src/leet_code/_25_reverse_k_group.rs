//! K 个一组翻转链表

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn reverse_k_group_v1(mut head: ListNodePtr, k: i32) -> ListNodePtr {
        let mut next_head = &mut head;
        for _ in 0..k {
            if let Some(node) = next_head.as_mut() {
                next_head = &mut node.next;
            } else {
                return head;
            }
        }

        let mut new_head = Self::reverse_k_group_v1(next_head.take(), k);
        for _ in 0..k {
            if let Some(mut node) = head {
                head = node.next.take();
                node.next = new_head.take();
                new_head = Some(node);
            }
        }
        new_head
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{create_list, Solution};

    #[test]
    fn test_reverse_k_group_v1() {
        assert_eq!(
            Solution::reverse_k_group_v1(create_list(vec![1, 2, 3, 4, 5]), 2),
            create_list(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group_v1(create_list(vec![1, 2, 3, 4, 5]), 3),
            create_list(vec![3, 2, 1, 4, 5])
        );
    }
}
