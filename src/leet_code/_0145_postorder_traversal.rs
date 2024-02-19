use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn postorder_traversal(root: TreeNodePtr) -> Vec<i32> {
        #[inline]
        fn dfs(root: &TreeNodePtr, ans: &mut Vec<i32>) {
            if root.is_some() {
                let node = root.as_ref().unwrap().borrow();
                dfs(&node.left, ans);
                dfs(&node.right, ans);
                ans.push(node.val);
            }
        }

        let mut ans = vec![];
        dfs(&root, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::postorder_traversal(linked_tree(
                1,
                None,
                linked_tree(2, linked_tree(3, None, None), None)
            )),
            vec![3, 2, 1]
        );
        assert_eq!(Solution::postorder_traversal(None), Vec::<i32>::new());
        assert_eq!(Solution::postorder_traversal(linked_tree(1, None, None)), vec![1]);
    }
}
