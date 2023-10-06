//! 数据反转 7 | 206

use super::{ListNodePtr, Solution};

impl Solution {
    /// 整数反转
    pub fn reverse(x: i32) -> i32 {
        x.abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
            * x.signum()
    }
}

impl Solution {
    /// 反转链表
    pub fn reverse_list_206_v1(head: ListNodePtr) -> ListNodePtr {
        Self::reverse_list(head, None)
    }

    fn reverse_list(head: ListNodePtr, prev: ListNodePtr) -> ListNodePtr {
        if let Some(mut node) = head {
            let tail = node.next.take();
            node.next = prev;
            return Self::reverse_list(tail, Some(node));
        }
        prev
    }
}

impl Solution {
    pub fn reverse_list_206_v2(head: ListNodePtr) -> ListNodePtr {
        let (mut res, mut node) = (None, head);
        while let Some(mut x) = node {
            node = x.next.take();
            x.next = res.take();
            res = Some(x);
        }
        res
    }
}
