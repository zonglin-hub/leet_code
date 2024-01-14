use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn level_order_bottom(root: TreeNodePtr) -> Vec<Vec<i32>> {
        fn dfs(node: &TreeNodePtr, level: usize, res: &mut Vec<Vec<i32>>) {
            if let Some(tree) = node {
                if res.len() < level + 1 {
                    res.push(Vec::new());
                }
                res[level].push(tree.borrow().val);
                dfs(&tree.borrow().left, level + 1, res);
                dfs(&tree.borrow().right, level + 1, res);
            }
        }

        let mut res = Vec::new();
        dfs(&root, 0, &mut res);
        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_level_order_bottom() {
        assert_eq!(
            Solution::level_order_bottom(linked_tree(
                3,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )),
            vec![vec![15, 7], vec![9, 20], vec![3]]
        );
        assert_eq!(Solution::level_order_bottom(linked_tree(1, None, None)), vec![vec![1]]);
        assert_eq!(Solution::level_order_bottom(None), Vec::<Vec<i32>>::new());
    }
}
