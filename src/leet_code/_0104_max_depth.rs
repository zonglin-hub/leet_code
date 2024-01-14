use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn max_depth(root: TreeNodePtr) -> i32 {
        fn dfs(root: &TreeNodePtr) -> i32 {
            match root {
                Some(tree) => {
                    let left_max = dfs(&tree.borrow().left);
                    let right_max = dfs(&tree.borrow().right);
                    1 + left_max.max(right_max)
                }
                None => 0,
            }
        }

        dfs(&root)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_max_depth() {
        assert_eq!(
            Solution::max_depth(linked_tree(
                3,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )),
            3
        );
        assert_eq!(Solution::max_depth(linked_tree(1, None, linked_tree(2, None, None))), 2);
    }
}
