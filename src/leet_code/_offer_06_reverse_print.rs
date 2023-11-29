//! 剑指 Offer 06. 从尾到头打印链表

use super::{ListNodePtr, Solution};

impl Solution {
    /// 这段代码实现了一个将链表逆序并打印的函数`reverse_print`。它接受一个链表的头节点指针作为输入，并返回一个包含链表逆序后所有节点值的向量。
    ///
    /// 首先，它使用一个空向量`res`来存储链表逆序后的节点值。然后，它使用一个循环来遍历链表，将每个节点的值添加到`res`中。在每次循环中，它将当前节点的下一个节点存储在`node`中，并将`node`向前移动一步。
    ///
    /// 接下来，它使用`reverse`方法来反转`res`。最后，它返回反转后的`res`。这样，函数就实现了将链表逆序并打印的功能。
    pub fn reverse_print(head: ListNodePtr) -> Vec<i32> {
        let mut res = vec![];
        let mut node = &head;

        while let Some(x) = node {
            res.push(x.val);
            node = &x.next;
        }

        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_reverse_print() {
        assert_eq!(
            Solution::reverse_print(linked_list!(1, 3, 2)),
            vec![2, 3, 1]
        );
    }
}
