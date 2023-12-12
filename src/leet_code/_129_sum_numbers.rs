use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn sum_numbers(root: TreeNodePtr) -> i32 {
        fn dfs(root: &TreeNodePtr, prev_sum: i32) -> i32 {
            if let Some(n) = root {
                let node = n.borrow();
                let sum = node.val + prev_sum * 10;

                if node.left.is_none() && node.right.is_none() {
                    return sum;
                } else {
                    return dfs(&node.left, sum) + dfs(&node.right, sum);
                }
            }
            0
        }

        dfs(&root, 0)
    }
}

#[cfg(test)]
mod test {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_sum_numbers() {
        assert_eq!(
            Solution::sum_numbers(linked_tree(
                1,
                linked_tree(2, None, None),
                linked_tree(3, None, None)
            )),
            25
        );
        assert_eq!(
            Solution::sum_numbers(linked_tree(
                4,
                linked_tree(9, linked_tree(5, None, None), linked_tree(1, None, None)),
                linked_tree(0, None, None)
            )),
            1026
        );
    }
}
