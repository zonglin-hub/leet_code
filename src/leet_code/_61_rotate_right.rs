use super::{ListNodePtr, Solution};

impl Solution {
    /// 这段代码实现了一个将链表向右旋转 k 个位置的函数`rotate_right`。它接受一个链表的头节点指针和一个整数 k 作为输入，并返回旋转后的链表头节点指针。
    ///
    /// 首先，它检查链表是否为空或 k 是否为 0。如果链表为空或 k 为 0，那么直接返回头节点指针。
    ///
    /// 然后，它计算链表的长度。它使用一个指针`ptr`来遍历链表，直到它遇到一个`None`节点。在每次循环中，它将指针`ptr`向前移动一步，并将`tail`设置为当前指针。最后，它使用`len`变量来记录链表的长度。
    ///
    /// 接下来，它计算要旋转的位置`k`。它使用取模运算符`%`来计算`k`在链表长度中的位置。如果`k`为 0，那么表示不需要旋转，因此直接返回头节点指针。
    ///
    /// 然后，它使用一个指针`ptr`来遍历链表，直到它到达要旋转的位置`k`。在每次循环中，它将指针`ptr`向前移动一步。然后，它将当前节点的下一个节点`ptr.next`取走，并将其存储在`new_head`变量中。
    ///
    /// 接下来，它使用模式匹配来处理不同的情况。如果`k`为 1，那么表示将链表向右旋转 1 个位置。
    /// 在这种情况下，它将`new_head`设置为链表的头节点，并将`new_head.next`设置为`head`。
    /// 如果`k`大于 1，那么表示将链表向右旋转 k 个位置。在这种情况下，它将`(*tail).next`设置为链表的头节点，并将`new_head`设置为`head`。
    ///
    /// 最后，它返回`new_head`作为旋转后的链表头节点指针。
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
