use std::collections::HashSet;

use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn is_unival_tree(root: TreeNodePtr) -> bool {
        fn pre_order(node: TreeNodePtr, set: &mut HashSet<i32>) {
            if let Some(node) = node {
                set.insert(node.borrow().val);
                pre_order(node.borrow_mut().left.take(), set);
                pre_order(node.borrow_mut().right.take(), set);
            }
        }
        let mut set = HashSet::new();
        pre_order(root, &mut set);
        set.len() == 1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_two_sum() {
        assert!(Solution::is_unival_tree(linked_tree(
            1,
            linked_tree(1, linked_tree(1, None, None), linked_tree(1, None, None)),
            linked_tree(1, None, linked_tree(1, None, None))
        )));
        assert!(!Solution::is_unival_tree(linked_tree(
            2,
            linked_tree(2, linked_tree(5, None, None), linked_tree(2, None, None)),
            linked_tree(2, None, None)
        )))
    }
}
