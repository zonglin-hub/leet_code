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

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut pre = None;
            let mut head = head;
            let mut next;

            while head.is_some() {
                next = head.as_mut().unwrap().next.take();
                head.as_mut().unwrap().next = pre.take();
                pre = head.take();
                head = next.take()
            }

            pre
        }

        if k == 1 {
            return head;
        }

        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;
        // let mut pre: *mut _ = &mut dummy;
        let mut pre = &mut dummy as *mut Option<Box<ListNode>>;
        // let mut end: *mut _ = &mut dummy;
        let mut end = &mut dummy as *mut Option<Box<ListNode>>;

        unsafe {
            while end.as_ref().unwrap().as_ref().unwrap().next.is_some() {
                for _ in 0..k {
                    if end.as_ref().unwrap().is_none() {
                        break;
                    }
                    end = &mut end.as_mut().unwrap().as_mut().unwrap().next;
                }
                if end.as_ref().unwrap().is_none() {
                    break;
                }

                let mut start = pre.as_mut().unwrap().as_mut().unwrap().next.take();
                pre = &mut dummy;

                while pre.as_ref().unwrap().as_ref().unwrap().next.is_some() {
                    pre = &mut pre.as_mut().unwrap().as_mut().unwrap().next;
                }

                let mut next = end.as_mut().unwrap().as_mut().unwrap().next.take();
                let startp: *mut _ = &mut start;
                end.as_mut().unwrap().as_mut().unwrap().next = None;
                pre.as_mut().unwrap().as_mut().unwrap().next = reverse(start).take();
                startp.as_mut().unwrap().as_mut().unwrap().next = next.take();
                pre = startp;
                end = pre;
            }
        }
        dummy.as_mut().unwrap().next.take()
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
    fn test_reverse_k_group() {
        assert_eq!(
            Solution::reverse_k_group(create_linked_list(&vec![1, 2, 3, 4, 5]), 2),
            create_linked_list(&vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(create_linked_list(&vec![1, 2, 3, 4, 5]), 3),
            create_linked_list(&vec![3, 2, 1, 4, 5])
        );
    }
}
