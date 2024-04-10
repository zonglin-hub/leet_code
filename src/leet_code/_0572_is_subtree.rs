use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn is_subtree(root: TreeNodePtr, sub_root: TreeNodePtr) -> bool {
        fn check(node: &TreeNodePtr, sub_root: &TreeNodePtr) -> bool {
            match (node, sub_root) {
                (None, None) => true,
                (Some(r), Some(s)) => {
                    if r.borrow().val != s.borrow().val {
                        return false;
                    }

                    check(&r.borrow().left, &s.borrow().left)
                        && check(&r.borrow().right, &s.borrow().right)
                }
                _ => false,
            }
        }

        fn dfs(root: &TreeNodePtr, sub_root: &TreeNodePtr) -> bool {
            if let Some(r) = root {
                check(root, sub_root)
                    || dfs(&r.borrow().left, sub_root)
                    || dfs(&r.borrow().right, sub_root)
            } else {
                false
            }
        }

        dfs(&root, &sub_root)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_two_sum() {
        assert!(Solution::is_subtree(
            linked_tree(
                3,
                linked_tree(4, linked_tree(1, None, None), linked_tree(2, None, None)),
                linked_tree(5, None, None)
            ),
            linked_tree(4, linked_tree(1, None, None), linked_tree(2, None, None))
        ));
        assert!(!Solution::is_subtree(
            linked_tree(
                3,
                linked_tree(
                    4,
                    linked_tree(1, None, None),
                    linked_tree(2, linked_tree(0, None, None), None)
                ),
                linked_tree(5, None, None)
            ),
            linked_tree(4, linked_tree(1, None, None), linked_tree(2, None, None))
        ));
    }
}
