use super::{ListNodePtr, Solution};

impl Solution {
    pub fn rotate_right(mut head: ListNodePtr, k: i32) -> ListNodePtr {
        if head.is_none() || k == 0 {
            return head;
        }

        let mut ptr = &mut head as *mut ListNodePtr;
        let mut tail = ptr;
        let mut len = 0;
        unsafe {
            while let Some(t) = ptr.as_mut().unwrap() {
                len += 1;
                tail = ptr;
                ptr = &mut t.next;
            }
        }

        let k = k % len;
        if k == 0 {
            return head;
        }

        let mut ptr = &mut head;
        for _ in 1..len - k {
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        let mut new_head = ptr.as_mut().unwrap().next.take();
        if k == 1 {
            new_head.as_mut().unwrap().next = head;
        } else {
            unsafe {
                (*tail).as_mut().unwrap().next = head;
            }
        }

        new_head
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{create_list, Solution};

    #[test]
    fn test_rotate_right() {
        assert_eq!(
            Solution::rotate_right(create_list(vec![1, 2, 3, 4, 5]), 3),
            create_list(vec![3, 4, 5, 1, 2])
        );

        assert_eq!(
            Solution::rotate_right(create_list(vec![0, 1, 2]), 4),
            create_list(vec![2, 0, 1])
        );
    }
}
