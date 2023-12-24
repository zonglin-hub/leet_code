use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    pub fn merge_nodes(mut head: ListNodePtr) -> ListNodePtr {
        let mut dummy = ListNode::new(-1);
        let mut tail = &mut dummy.next;
        let mut total = 0;
        while let Some(cur) = head {
            if cur.val == 0 && total > 0 {
                *tail = Some(Box::new(ListNode::new(total)));
                tail = &mut tail.as_mut()?.next;
                total = 0;
            } else {
                total += cur.val;
            }
            head = cur.next;
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_merge_nodes() {
        assert_eq!(
            Solution::merge_nodes(linked_list!(0, 3, 1, 0, 4, 5, 2, 0)),
            linked_list!(4, 11)
        );
        assert_eq!(
            Solution::merge_nodes(linked_list!(0, 1, 0, 3, 0, 2, 2, 0)),
            linked_list!(1, 3, 4)
        );
    }
}
