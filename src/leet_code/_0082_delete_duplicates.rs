use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    /// 这个函数是用于删除链表中的重复元素。它接受一个链表的头节点作为参数，并返回一个新链表的头节点，其中重复的元素已被删除。
    ///
    /// 函数的主要逻辑如下：
    ///
    /// 1. 创建一个临时节点 `dummy`，并将其初始化为一个新的链表节点，其值为0。这个临时节点用于构建新的链表。
    /// 2. 创建一个可变引用 `dummy_next`，指向临时节点的 `next` 字段。这个引用用于在遍历原链表时，将不重复的元素连接到新链表上。
    /// 3. 创建一个可变的 `last` 变量，用于存储上一个遍历到的元素的值。初始值为 `None`。
    /// 4. 进入循环，直到原链表的头节点为 `None`。循环内部的操作如下：
    /// - a. 将原链表的头节点赋值给 `x`，并将 `x` 从原链表中移除，即 `head = x.next.take()`。
    /// - b. 取出 `x` 节点的值，并将其存储在 `t` 变量中。
    /// - c. 判断 `t` 是否与 `last` 相等，或者 `head` 指向的节点的值是否与 `t` 相等。如果不满足这两个条件，说明 `x` 是一个非重复节点，需要将其连接到新链表的末尾。通过 `r.get_or_insert(x)` 将 `x` 插入到新链表的末尾，并更新 `r` 的指向。
    /// - d. 更新 `last` 的值为 `Some(t)`，表示已经遍历过一个值为 `t` 的节点。
    /// 5. 循环结束后，返回新链表的头节点，即 `dummy.next`。
    ///
    /// 这样，函数最终返回的是一个新链表，其中重复的元素已经被删除。原链表中的非重复元素按照原来的顺序连接在新链表中。
    pub fn delete_duplicates_82(mut head: ListNodePtr) -> ListNodePtr {
        let mut dummy = ListNode::new(0);
        let mut dummy_next = &mut dummy.next;
        let mut last = None;
        while let Some(mut node) = head {
            head = node.next.take();
            let node_val = node.val;
            if Some(node_val) != last && head.as_ref().map_or(true, |next| next.val != node_val) {
                dummy_next = &mut dummy_next.get_or_insert(node).next;
            }
            last = Some(node_val);
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        leet_code::{ListNode, Solution},
        linked_list,
    };

    #[test]
    fn test_delete_duplicates_82() {
        assert_eq!(
            Solution::delete_duplicates_82(linked_list!(1, 2, 3, 3, 4, 4, 5)),
            linked_list!(1, 2, 5)
        );
        assert_eq!(Solution::delete_duplicates_82(linked_list!(1, 1, 1, 2, 3)), linked_list!(2, 3));
        assert_eq!(Solution::delete_duplicates_82(None), None);
    }
}
