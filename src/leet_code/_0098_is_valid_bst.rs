use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn is_valid_bst(root: TreeNodePtr) -> bool {
        fn verify(root: TreeNodePtr, lower: Option<i32>, upper: Option<i32>) -> bool {
            match root {
                Some(n) => {
                    lower.is_none_or(|x| x < n.borrow().val)
                        && upper.is_none_or(|x| n.borrow().val < x)
                        && verify(n.borrow().left.clone(), lower, Some(n.borrow().val))
                        && verify(n.borrow().right.clone(), Some(n.borrow().val), upper)
                }
                None => true,
            }
        }

        verify(root, None, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_is_valid_bst() {
        assert!(Solution::is_valid_bst(linked_tree(
            2,
            linked_tree(1, None, None),
            linked_tree(3, None, None)
        )));
        assert!(!Solution::is_valid_bst(linked_tree(
            5,
            linked_tree(1, None, None),
            linked_tree(4, linked_tree(3, None, None), linked_tree(6, None, None))
        )));
    }
}
