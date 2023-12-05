use crate::leet_code::ListNode;

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn sort_list(head: ListNodePtr) -> ListNodePtr {
        let mut cur = head;
        let mut v = vec![];

        while let Some(node) = cur {
            v.push(node.val);
            cur = node.next;
        }

        v.sort_unstable();
        let mut prev = None;

        while let Some(node) = v.pop() {
            let mut node = ListNode::new(node);
            node.next = prev;
            prev = Some(Box::new(node));
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::{leet_code::Solution, linked_list};

    #[test]
    fn test_sort_list() {
        assert_eq!(
            Solution::sort_list(linked_list!(4, 2, 1, 3)),
            linked_list!(1, 2, 3, 4)
        );
        assert_eq!(
            Solution::sort_list(linked_list!(-1, 5, 3, 4, 0)),
            linked_list!(-1, 0, 3, 4, 5)
        );
        assert_eq!(Solution::sort_list(None), None);
    }
}
