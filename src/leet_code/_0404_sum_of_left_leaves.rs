use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn sum_of_left_leaves(root: TreeNodePtr) -> i32 {
        fn dfs(node: &TreeNodePtr, original_state: bool, sum: &mut i32) {
            if let Some(node) = node {
                let v = node.borrow().val;
                if node.borrow().left.is_none() && node.borrow().right.is_none() && original_state {
                    *sum += v;
                } else {
                    dfs(&node.borrow_mut().left, true, sum);
                    dfs(&node.borrow_mut().right, false, sum);
                }
            }
        }

        let mut sum = 0;
        dfs(&root, false, &mut sum);
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::sum_of_left_leaves(linked_tree(
                3,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )),
            24
        );
        assert_eq!(Solution::sum_of_left_leaves(linked_tree(1, None, None)), 0);
    }
}
