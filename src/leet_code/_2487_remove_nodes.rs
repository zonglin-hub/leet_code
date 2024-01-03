use super::{ListNodePtr, Solution};

impl Solution {
    pub fn remove_nodes(head: ListNodePtr) -> ListNodePtr {
        let mut stk = Vec::<ListNodePtr>::new();
        let mut node = head;
        while node.is_some() {
            while matches!(stk.last(), Some(pre) if pre.as_ref()?.val < node.as_ref()?.val) {
                stk.pop();
            }
            let next = node.as_mut()?.next.take();
            stk.push(node);
            node = next;
        }
        let mut head = stk[0].take();
        let mut node = &mut head.as_mut()?.next;
        for i in stk.iter_mut().skip(1) {
            *node = i.take();
            node = &mut node.as_mut()?.next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_remove_nodes() {
        assert_eq!(
            Solution::remove_nodes(linked_list!(5, 2, 13, 3, 8)),
            linked_list!(13, 8)
        );
        assert_eq!(
            Solution::remove_nodes(linked_list!(1, 1, 1, 1)),
            linked_list!(1, 1, 1, 1)
        );
    }
}
