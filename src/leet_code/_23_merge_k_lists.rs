//! 合并 K 个升序链表

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::{ListNode, ListNodePtr, Solution};

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    /// 这个函数的实现思路是使用优先队列（`BinaryHeap`）来合并多个有序链表。
    ///
    /// 首先，将所有链表的头节点放入优先队列中。然后，从优先队列中取出最小的节点，并将其连接到合并后的链表的头部。
    /// 接着，将取出的节点的下一个节点放入优先队列中，重复这个过程，直到优先队列为空。最后，返回合并后的链表的头节点。
    ///
    /// 在这个实现中，我们使用了`Ord`和`PartialOrd`特性来实现链表节点的比较。由于链表节点的值是整数，我们可以直接使用整数的比较运算符来进行比较。
    pub fn merge_k_lists(lists: Vec<ListNodePtr>) -> ListNodePtr {
        let mut priority_queue = BinaryHeap::new();
        for head_node in lists {
            priority_queue.push(head_node);
        }

        let mut merged_list = ListNode::new(-1);
        let mut ptr = &mut merged_list;

        while !priority_queue.is_empty() {
            let mut min_node = priority_queue.pop().unwrap();
            if let Some(node) = min_node.as_mut() {
                priority_queue.push(node.next.take());
                ptr.next = min_node;
                ptr = ptr.next.as_mut().unwrap();
            }
        }
        merged_list.next
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::ListMaker;
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::list;

    #[test]
    fn test_merge_k_lists() {
        assert_eq!(Solution::merge_k_lists(vec![]), None);
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
        assert_eq!(
            Solution::merge_k_lists(vec![list!(1, 4, 5, 8), list!(1, 2, 3, 3, 4), list!(2, 6),]),
            list!(1, 1, 2, 2, 3, 3, 4, 4, 5, 6, 8)
        );
        assert_eq!(
            Solution::merge_k_lists(vec![list!(1, 4, 5), list!(1, 3, 4), list!(2, 6)]),
            list!(1, 1, 2, 3, 4, 4, 5, 6)
        );
    }
}
