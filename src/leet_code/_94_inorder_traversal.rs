use std::iter;

use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn inorder_traversal(root: TreeNodePtr) -> Vec<i32> {
        fn inorder(x: &TreeNodePtr) -> Box<dyn iter::Iterator<Item = i32>> {
            match x {
                None => Box::new(iter::empty()),
                Some(rc) => {
                    let tree = rc.borrow();
                    Box::new(
                        inorder(&tree.left).chain(iter::once(tree.val)).chain(inorder(&tree.right)),
                    )
                }
            }
        }

        inorder(&root).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_inorder_traversal() {
        assert_eq!(Solution::inorder_traversal(None), Vec::<i32>::new());
        assert_eq!(Solution::inorder_traversal(linked_tree(1, None, None)), vec![1]);
        assert_eq!(
            Solution::inorder_traversal(linked_tree(
                1,
                None,
                linked_tree(2, linked_tree(3, None, None), None)
            )),
            vec![1, 3, 2]
        );
    }
}
