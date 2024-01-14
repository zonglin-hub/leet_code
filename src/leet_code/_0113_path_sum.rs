use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn path_sum(root: TreeNodePtr, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(
            root: &TreeNodePtr,
            pre_sum: i32,
            target_sum: i32,
            tmp: &mut Vec<i32>,
            ret: &mut Vec<Vec<i32>>,
        ) {
            if let Some(root) = root {
                let tree = root.borrow();
                if tree.left.is_none() && tree.right.is_none() {
                    if pre_sum + tree.val == target_sum {
                        tmp.push(tree.val);
                        ret.push(tmp.clone());
                        tmp.pop();
                    }
                    return;
                }

                tmp.push(tree.val);
                dfs(&tree.left, pre_sum + tree.val, target_sum, tmp, ret);
                dfs(&tree.right, pre_sum + tree.val, target_sum, tmp, ret);
                tmp.pop();
            }
        }

        let mut ret = vec![];
        dfs(&root, 0, target_sum, &mut vec![], &mut ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_path_sum() {
        assert_eq!(
            Solution::path_sum(
                linked_tree(1, linked_tree(2, None, None), linked_tree(3, None, None)),
                5
            ),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::path_sum(linked_tree(1, linked_tree(2, None, None), None), 0),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::path_sum(
                linked_tree(
                    5,
                    linked_tree(
                        4,
                        linked_tree(11, linked_tree(7, None, None), linked_tree(2, None, None)),
                        None
                    ),
                    linked_tree(
                        8,
                        linked_tree(13, None, None),
                        linked_tree(4, linked_tree(5, None, None), linked_tree(1, None, None))
                    )
                ),
                22
            ),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }
}
