use super::{ListNodePtr, Solution};

impl Solution {
    /// 这个函数也是用于删除链表中的重复元素。该函数也接受链表的头节点作为参数，并返回处理后的链表的头节点。
    ///
    /// 函数的主要逻辑如下：
    ///
    /// 1. 首先判断链表是否为空，如果是空链表，则直接返回头节点（即 `None`）。
    /// 2. 初始化 `node` 变量为链表的头节点，并将 `duplicate` 变量设置为头节点的值。
    /// 3. 进入循环，循环条件为 `node.next.take()`，即遍历链表直到 `node` 的下一个节点为 `None`。循环内部的操作如下：
    /// - a. 判断当前节点的下一个节点 `x` 的值是否与 `duplicate` 相等。
    /// - b. 如果相等，则将当前节点的 `next` 指向 `x` 的下一个节点，从而删除重复节点 `x`。
    /// - c. 如果不相等，更新 `duplicate` 的值为 `x.val`，将当前节点的 `next` 指向 `x`，并将 `node` 移动到下一个节点。
    /// 4. 循环结束后，返回头节点 `head`。
    ///
    /// 这个函数通过遍历链表，将重复的元素删除，只保留第一个出现的元素。最终返回处理后的链表头节点。需要注意的是，在遍历过程中，通过更新 `node` 变量来移动到下一个节点，并且使用 `take()` 方法取消原链表中节点的所有权，从而实现删除操作。
    pub fn delete_duplicates_83(mut head: ListNodePtr) -> ListNodePtr {
        let mut node = head.as_mut()?;
        let mut duplicate = node.val;
        while let Some(x) = node.next.take() {
            if x.val == duplicate {
                node.next = x.next
            } else {
                duplicate = x.val;
                node.next = Some(x);
                node = node.next.as_mut()?;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        leet_code::{ListNode, Solution},
        linked_list,
    };

    #[test]
    fn test_delete_duplicates() {
        assert_eq!(Solution::delete_duplicates_83(linked_list!(1, 1, 2)), linked_list!(1, 2));
        assert_eq!(
            Solution::delete_duplicates_83(linked_list!(1, 1, 2, 3, 3)),
            linked_list!(1, 2, 3)
        );
        assert_eq!(Solution::delete_duplicates_83(None), None);
    }
}
