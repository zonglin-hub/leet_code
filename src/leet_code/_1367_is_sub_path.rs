use super::{ListNodePtr, Solution, TreeNodePtr};

impl Solution {
    #[allow(clippy::borrowed_box)]
    pub fn is_sub_path(head: ListNodePtr, root: TreeNodePtr) -> bool {
        fn is_same(head: &ListNodePtr, root: TreeNodePtr) -> bool {
            match (head, root) {
                (None, _) => true,
                (_, None) => false,
                (Some(head), Some(root)) => {
                    let root_ref = root.borrow();
                    let head_ref = &head.next;

                    head.val == root_ref.val
                        && (is_same(head_ref, root_ref.left.clone())
                            || is_same(head_ref, root_ref.right.clone()))
                }
            }
        }

        fn dfs(head: &ListNodePtr, root: TreeNodePtr) -> bool {
            match (head, root) {
                (None, _) | (_, None) => false,
                (Some(temp_head), Some(root)) => {
                    let root_ref = root.borrow();

                    if temp_head.val == root_ref.val && is_same(head, Some(root.clone())) {
                        return true;
                    }

                    dfs(head, root_ref.left.clone()) || dfs(head, root_ref.right.clone())
                }
            }
        }

        dfs(&head, root)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::linked_tree;
    use crate::leet_code::ListNode;
    use crate::leet_code::Solution;
    use crate::linked_list;

    #[test]
    fn test_is_sub_path() {
        let tree = linked_tree(
            1,
            linked_tree(4, None, linked_tree(2, linked_tree(1, None, None), None)),
            linked_tree(
                4,
                linked_tree(
                    2,
                    linked_tree(6, None, None),
                    linked_tree(8, linked_tree(1, None, None), linked_tree(3, None, None)),
                ),
                None,
            ),
        );
        assert!(Solution::is_sub_path(linked_list!(4, 2, 8), tree.clone()));
        assert!(Solution::is_sub_path(linked_list!(1, 4, 2, 6), tree.clone()));
        assert!(!Solution::is_sub_path(linked_list!(1, 4, 2, 6, 8), tree.clone()));
    }
}
