use super::{ListNode, ListNodePtr, Solution};

impl Solution {
    /// 这个函数是一个用于将一个链表按照某个值 `x` 进行分区的函数。它会重新排列链表中的节点，使得所有小于 `x` 的节点都出现在大于或等于 `x` 的节点之前，同时保持节点的相对顺序不变。函数会返回新链表的头节点。
    ///
    /// 函数的参数包括一个可变的 `head`，表示链表的头节点，和一个整数 `x`，用于确定分区的值。
    ///
    /// 函数首先创建两个虚拟节点 `dh1` 和 `dh2`，它们分别用于表示小于 `x` 的节点链表和大于等于 `x` 的节点链表的头节点。这两个节点初始时的值并不重要，因为它们仅仅用作哨兵节点，真正的数据节点将会连接在这两个哨兵节点之后。
    ///
    /// 然后函数进入一个循环，循环遍历原链表中的每个节点。对于每个节点，函数首先将其从原链表中移除，然后根据节点的值决定将节点连接到 `dh1` 后面还是 `dh2` 后面。如果节点的值小于 `x`，则连接到 `dh1` 后面；否则连接到 `dh2` 后面。
    /// 连接节点后，相应的哨兵节点的指针会向前移动，指向新连接的节点。
    ///
    /// 循环结束后，函数会将 `dh2` 的后面连接的节点连接到 `dh1` 的后面，然后将 `dh1` 的 `next` 指针作为结果返回。
    ///
    /// 这样，函数就完成了链表的分区操作。小于 `x` 的节点都出现在了大于等于 `x` 的节点之前，且节点的相对顺序没有改变。
    pub fn partition(mut head: ListNodePtr, x: i32) -> ListNodePtr {
        let (mut dh1, mut dh2) = (Box::new(ListNode::new(0)), Box::new(ListNode::new(0)));
        let (mut dh1_mut, mut dh2_mut) = (dh1.as_mut(), dh2.as_mut());

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                dh1_mut.next = Some(node);
                dh1_mut = dh1_mut.next.as_mut()?;
            } else {
                dh2_mut.next = Some(node);
                dh2_mut = dh2_mut.next.as_mut()?;
            }
        }

        dh1_mut.next = dh2.next;
        dh1.next
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        leet_code::{ListNode, Solution},
        linked_list,
    };

    #[test]
    fn test_partition() {
        assert_eq!(
            Solution::partition(linked_list!(1, 4, 3, 2, 5, 2), 3),
            linked_list!(1, 2, 2, 4, 3, 5)
        );
        assert_eq!(Solution::partition(linked_list!(2, 1), 2), linked_list!(1, 2));
    }
}
