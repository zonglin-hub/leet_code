use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    pub fn insertion_sort_list_safe(mut head: ListNodePtr) -> ListNodePtr {
        let mut dummy = ListNode::new(0);
        while let Some(mut node) = head {
            head = node.next.take();
            let mut ptr = &mut dummy;
            while let Some(n) = &ptr.next {
                if n.val >= node.val {
                    break;
                }
                ptr = ptr.next.as_mut()?;
            }
            node.next = ptr.next.take();
            ptr.next = Some(node);
        }
        dummy.next
    }

    pub fn insertion_sort_list_unsafe(mut head: ListNodePtr) -> ListNodePtr {
        let mut dummy = ListNode::new(0);
        while let Some(mut node) = head {
            head = node.next.take();
            unsafe {
                let mut ptr = &mut dummy as *mut ListNode;
                while let Some(n) = (*ptr).next.as_mut() {
                    if n.val >= node.val {
                        break;
                    }
                    ptr = n.as_mut() as *mut ListNode;
                }
                node.next = (*ptr).next.take();
                (*ptr).next = Some(node);
            }
        }
        dummy.next.take()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_insertion_sort_list_safe() {
        assert_eq!(
            Solution::insertion_sort_list_safe(linked_list!(4, 2, 1, 3)),
            linked_list!(1, 2, 3, 4)
        );
        assert_eq!(
            Solution::insertion_sort_list_safe(linked_list!(-1, 5, 3, 4, 0)),
            linked_list!(-1, 0, 3, 4, 5)
        );
    }

    #[test]
    fn test_insertion_sort_list_unsafe() {
        assert_eq!(
            Solution::insertion_sort_list_unsafe(linked_list!(4, 2, 1, 3)),
            linked_list!(1, 2, 3, 4)
        );
        assert_eq!(
            Solution::insertion_sort_list_unsafe(linked_list!(-1, 5, 3, 4, 0)),
            linked_list!(-1, 0, 3, 4, 5)
        );
    }
}
