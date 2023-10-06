//! K 个一组翻转链表

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn reverse_k_group_v1(mut head: ListNodePtr, k: i32) -> ListNodePtr {
        let mut next_head = &mut head;
        // 获取下一轮头结点
        for _ in 0..k {
            if let Some(node) = next_head.as_mut() {
                next_head = &mut node.next;
            } else {
                return head;
            }
        }
        // 获取除本轮结果
        let mut new_head = Self::reverse_k_group_v1(next_head.take(), k);
        // 翻转本轮k个节点
        for _ in 0..k {
            if let Some(mut node) = head {
                head = node.next.take();
                node.next = new_head.take();
                new_head = Some(node);
            }
        }
        new_head
    }
}
