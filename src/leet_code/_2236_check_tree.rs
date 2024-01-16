use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn check_tree(root: TreeNodePtr) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            let left = node.left.as_ref().unwrap().borrow();
            let right = node.right.as_ref().unwrap().borrow();
            left.val + right.val == node.val
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_check_tree() {
        assert!(Solution::check_tree(linked_tree(
            10,
            linked_tree(4, None, None),
            linked_tree(6, None, None)
        )));
        assert!(!Solution::check_tree(linked_tree(
            5,
            linked_tree(3, None, None),
            linked_tree(1, None, None)
        )));
    }
}
