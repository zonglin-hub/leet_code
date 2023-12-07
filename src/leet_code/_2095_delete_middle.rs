use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    pub fn delete_middle(head: ListNodePtr) -> ListNodePtr {
        // let mut dummy = Box::new(ListNode { val: 0, next: head });
        // let mut fast = &mut dummy as *mut Box<ListNode>;
        // let mut slow = &mut dummy as *mut Box<ListNode>;

        // unsafe {
        //     while (*fast).next.as_ref().is_some() && (*fast).next.as_ref().unwrap().next.is_some() {
        //         fast = (*fast).next.as_mut().unwrap().next.as_mut().unwrap();
        //         slow = (*slow).next.as_mut().unwrap();
        //     }
        //     (*slow).next = (*slow).next.take().unwrap().next;
        // }

        // dummy.next

        unsafe {
            let dummy = &mut ListNode { val: 0, next: head } as *mut ListNode;
            let mut fast = dummy;
            let mut slow = dummy;

            while (*fast).next.is_some() && (*fast).next.as_mut()?.next.is_some() {
                fast = (*fast).next.as_mut()?.as_mut().next.as_mut()?.as_mut();
                slow = (*slow).next.as_mut()?.as_mut();
            }
            (*slow).next = (*slow).next.take()?.next;
            (*dummy).next.to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::{leet_code::Solution, linked_list};

    #[test]
    fn test_delete_middle() {
        assert_eq!(Solution::delete_middle(linked_list!(2, 1)), linked_list!(2));
        assert_eq!(
            Solution::delete_middle(linked_list!(1, 3, 4, 7, 1, 2, 6)),
            linked_list!(1, 3, 4, 1, 2, 6)
        );
        assert_eq!(
            Solution::delete_middle(linked_list!(1, 2, 3, 4)),
            linked_list!(1, 2, 4)
        );
    }
}
