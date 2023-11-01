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
            // 避免最后一个节点被take后，tail指针失效的问题
            new_head.as_mut().unwrap().next = head;
        } else {
            unsafe {
                (*tail).as_mut().unwrap().next = head;
            }
        }

        new_head
    }
}
