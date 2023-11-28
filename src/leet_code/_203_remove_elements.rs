//! 移除链表元素
//!
//! 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。

use super::{ListNodePtr, Solution};

impl Solution {
    /// 这段代码实现了一个删除链表中指定值的函数`remove_elements`。它接受一个链表的头节点指针和一个要删除的值作为输入，并返回删除后的链表头节点指针。
    ///
    /// 首先，它使用模式匹配来处理头节点指针。如果头节点指针为空，那么表示链表为空，因此直接返回空指针。否则，它创建一个局部变量`node`，并将其设置为头节点。
    ///
    /// 然后，它使用模式匹配来检查当前节点的值是否等于要删除的值。
    /// 如果是，那么表示当前节点是要删除的节点，因此它将头节点指针指向当前节点的下一个节点，并将当前节点的下一个节点设置为`Self::remove_elements(node.next, val)`。
    /// 如果当前节点的值不等于要删除的值，那么表示当前节点不是要删除的节点，因此它将头节点指针指向当前节点的下一个节点，并将当前节点的下一个节点设置为`Self::remove_elements(node.next, val)`。
    ///
    /// 最后，它返回头节点指针。这样，函数就实现了删除链表中指定值的功能。
    pub fn remove_elements(head: ListNodePtr, val: i32) -> ListNodePtr {
        match head {
            None => None,
            Some(mut node) => match node.val == val {
                true => Self::remove_elements(node.next, val),
                false => {
                    node.next = Self::remove_elements(node.next, val);
                    Some(node)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{create_list, Solution};

    #[test]
    fn test_remove_elements() {
        assert_eq!(
            Solution::remove_elements(create_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
            create_list(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(Solution::remove_elements(None, 1), None);
        assert_eq!(
            Solution::remove_elements(create_list(vec![7, 7, 7, 7]), 7),
            None
        );
    }
}
