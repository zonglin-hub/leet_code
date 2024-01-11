use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn invert_tree(root: TreeNodePtr) -> TreeNodePtr {
        if let Some(root) = root {
            let left = Self::invert_tree(root.borrow_mut().left.take());
            let right = Self::invert_tree(root.borrow_mut().right.take());
            root.borrow_mut().left = right;
            root.borrow_mut().right = left;
            Some(root)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_invert_tree() {
        assert_eq!(
            Solution::invert_tree(linked_tree(
                4,
                linked_tree(2, linked_tree(1, None, None), linked_tree(3, None, None)),
                linked_tree(7, linked_tree(6, None, None), linked_tree(9, None, None))
            )),
            linked_tree(
                4,
                linked_tree(7, linked_tree(9, None, None), linked_tree(6, None, None)),
                linked_tree(2, linked_tree(3, None, None), linked_tree(1, None, None))
            )
        );
        assert_eq!(
            Solution::invert_tree(linked_tree(
                2,
                linked_tree(1, None, None),
                linked_tree(3, None, None)
            )),
            linked_tree(2, linked_tree(3, None, None), linked_tree(1, None, None))
        );
        assert_eq!(Solution::invert_tree(None), None);
    }
}
