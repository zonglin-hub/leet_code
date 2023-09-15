//! 合并 K 个升序链表

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::{ListNode, Solution};

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
    pub fn merge_k_lists_v1(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::new();
        for i in lists {
            min_heap.push(i);
        }

        let mut dummy = ListNode::new(-1);
        let mut ptr = &mut dummy;

        while !min_heap.is_empty() {
            let mut node = min_heap.pop().expect("");
            if let Some(n) = node.as_mut() {
                min_heap.push(n.next.take());
                ptr.next = node;
                ptr = ptr.next.as_mut().expect("");
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {

    use crate::create_list;

    use super::*;

    #[test]
    fn test_merge_k_lists_v1() {
        /*
            输入：lists = [[1,4,5],[1,3,4],[2,6]]
            输出：[1,1,2,3,4,4,5,6]
            解释：链表数组如下：
            [
            1->4->5,
            1->3->4,
            2->6
            ]
            将它们合并到一个有序链表中得到。
            1->1->2->3->4->4->5->6
        */
        let expectation = vec![
            create_list(vec![1, 4, 5]),
            create_list(vec![1, 3, 4]),
            create_list(vec![2, 6]),
        ];
        let expect = create_list(vec![1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(Solution::merge_k_lists_v1(expectation), expect);

        // let expectation = None;

        /*
            输入：lists = []
            输出：[]
        */
        assert_eq!(Solution::merge_k_lists_v1(vec![]), None);

        /*
            输入：lists = [[]]
            输出：[]
        */
        assert_eq!(Solution::merge_k_lists_v1(vec![None]), None);
    }
}
