//! 回文链表
//!

use super::{ListNodePtr, Solution};

impl Solution {
    /// 这段代码实现了一个判断链表是否为回文的函数`is_palindrome`。它接受一个链表的头节点指针作为输入，并返回一个布尔值，表示链表是否为回文。
    ///
    /// 首先，它使用一个变量`val`来存储链表的所有值。然后，它使用一个循环来遍历链表，将每个节点的值添加到`val`中。在每次循环中，它将当前节点的下一个节点存储在`node`中，并将`node`向前移动一步。
    ///
    /// 接下来，它使用一个变量`val_rev`来存储链表的逆序。它使用`clone`方法来复制`val`，并使用`reverse`方法来反转`val`。
    /// 然后，它使用模式匹配来比较`val`和`val_rev`是否相等。如果相等，那么表示链表是回文，因此返回`true`。如果不相等，那么表示链表不是回文，因此返回`false`。
    ///
    /// 最后，它返回比较的结果。这样，函数就实现了判断链表是否为回文的功能。
    pub fn is_palindrome(head: ListNodePtr) -> bool {
        let (mut val, mut node) = (vec![], &head);
        while let Some(node_box) = node {
            val.push(node_box.val);
            node = &node_box.next;
        }

        let val_rev = val.clone();
        val.reverse();
        val == val_rev
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(linked_list!(1, 2)), false);
    }
}
