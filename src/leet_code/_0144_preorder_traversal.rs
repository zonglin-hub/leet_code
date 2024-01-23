use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn preorder_traversal(root: TreeNodePtr) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(tree) = node {
                let mut tree = tree.borrow_mut();
                ans.push(tree.val);
                stack.push(tree.right.take());
                stack.push(tree.left.take());
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::preorder_traversal(linked_tree(
                1,
                None,
                linked_tree(2, linked_tree(3, None, None), None)
            )),
            vec![1, 2, 3]
        );
        assert_eq!(Solution::preorder_traversal(None), Vec::<i32>::new());
        assert_eq!(Solution::preorder_traversal(linked_tree(1, None, None)), vec![1]);
        assert_eq!(
            Solution::preorder_traversal(linked_tree(1, linked_tree(2, None, None), None)),
            vec![1, 2]
        );
        assert_eq!(
            Solution::preorder_traversal(linked_tree(1, None, linked_tree(2, None, None))),
            vec![1, 2]
        );
    }
}
