use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn is_same_tree(p: TreeNodePtr, q: TreeNodePtr) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val
                    && Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_is_same_tree() {
        let root = linked_tree(1, linked_tree(2, None, None), linked_tree(3, None, None));
        assert!(Solution::is_same_tree(root.clone(), root.clone()));
        assert_eq!(
            Solution::is_same_tree(
                linked_tree(1, linked_tree(2, None, None), None),
                linked_tree(1, None, linked_tree(2, None, None))
            ),
            false
        );
        assert_eq!(
            Solution::is_same_tree(
                linked_tree(1, linked_tree(2, None, None), linked_tree(1, None, None)),
                linked_tree(1, linked_tree(1, None, None), linked_tree(2, None, None))
            ),
            false
        );
    }
}
