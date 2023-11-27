//! 剑指 Offer 06. 从尾到头打印链表

use super::{ListNodePtr, Solution};

impl Solution {
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
    use crate::leet_code::{create_list, Solution};

    #[test]
    fn test_reverse_print() {
        assert_eq!(
            Solution::reverse_print(create_list(vec![1, 3, 2])),
            vec![2, 3, 1]
        );
    }
}
