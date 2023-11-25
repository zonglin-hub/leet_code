use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    pub fn reverse_between(head: ListNodePtr, left: i32, right: i32) -> ListNodePtr {
        let mut dummy_node = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));

        let mut pre = &mut dummy_node;
        for _ in 0..left - 1 {
            pre = &mut pre.as_mut().unwrap().next;
        }

        let mut cur = pre.as_mut().unwrap().next.take();
        let mut next;

        for _ in 0..(right - left) {
            next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = pre.as_mut().unwrap().next.take();
            pre.as_mut().unwrap().next = next.take();
        }

        while pre.as_mut().unwrap().next.is_some() {
            pre = &mut pre.as_mut().unwrap().next;
        }
        pre.as_mut().unwrap().next = cur.take();
        dummy_node.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{create_list, Solution};

    #[test]
    fn test_reverse_between() {
        assert_eq!(
            Solution::reverse_between(create_list(vec![1, 2, 3, 4, 5]), 2, 4),
            create_list(vec![1, 4, 3, 2, 5])
        );

        assert_eq!(
            Solution::reverse_between(create_list(vec![5]), 1, 1),
            create_list(vec![5])
        );
    }
}
