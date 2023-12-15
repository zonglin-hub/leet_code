use super::{Solution, TreeNodePtr};

impl Solution {
    /// 深度优先搜索 (力扣官方题解)
    pub fn reverse_odd_levels(root: TreeNodePtr) -> TreeNodePtr {
        fn dfs(r1: &TreeNodePtr, r2: &TreeNodePtr, is_odd: bool) {
            if let (Some(r1), Some(r2)) = (r1, r2) {
                let (mut r1, mut r2) = (r1.borrow_mut(), r2.borrow_mut());
                if is_odd {
                    std::mem::swap(&mut r1.val, &mut r2.val);
                }
                dfs(&r1.left, &r2.right, !is_odd);
                dfs(&r1.right, &r2.left, !is_odd);
            }
        }
        let r = root?;
        dfs(&r.borrow().left, &r.borrow().right, true);
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_reverse_odd_levels() {
        assert_eq!(
            Solution::reverse_odd_levels(linked_tree(
                2,
                linked_tree(3, linked_tree(8, None, None), linked_tree(13, None, None)),
                linked_tree(5, linked_tree(21, None, None), linked_tree(34, None, None))
            )),
            linked_tree(
                2,
                linked_tree(5, linked_tree(8, None, None), linked_tree(13, None, None)),
                linked_tree(3, linked_tree(21, None, None), linked_tree(34, None, None))
            )
        );
        assert_eq!(
            Solution::reverse_odd_levels(linked_tree(
                7,
                linked_tree(13, None, None),
                linked_tree(11, None, None)
            )),
            linked_tree(7, linked_tree(11, None, None), linked_tree(13, None, None))
        );
    }
}
