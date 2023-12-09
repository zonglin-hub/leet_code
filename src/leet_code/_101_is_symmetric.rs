use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn is_symmetric(root: TreeNodePtr) -> bool {
        fn is_same_tree(p: TreeNodePtr, q: TreeNodePtr) -> bool {
            match (p, q) {
                (Some(p), Some(q)) => {
                    p.borrow().val == q.borrow().val
                        && is_same_tree(p.borrow().left.clone(), q.borrow().right.clone())
                        && is_same_tree(p.borrow().right.clone(), q.borrow().left.clone())
                }
                (None, None) => true,
                _ => false,
            }
        }

        is_same_tree(
            root.clone().unwrap().borrow().left.clone(),
            root.unwrap().borrow().right.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_is_symmetric() {
        assert!(Solution::is_symmetric(linked_tree(
            1,
            linked_tree(2, linked_tree(3, None, None), linked_tree(4, None, None)),
            linked_tree(2, linked_tree(4, None, None), linked_tree(3, None, None))
        )));
        assert_eq!(
            Solution::is_symmetric(linked_tree(
                1,
                linked_tree(2, None, linked_tree(3, None, None)),
                linked_tree(2, None, linked_tree(3, None, None))
            )),
            false
        );
    }
}
