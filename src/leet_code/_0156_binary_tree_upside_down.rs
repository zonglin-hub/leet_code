use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn upside_down_binary_tree(root: TreeNodePtr) -> TreeNodePtr {
        #[inline]
        fn preorder(root: TreeNodePtr, left: TreeNodePtr, right: TreeNodePtr) -> TreeNodePtr {
            if let Some(node) = root {
                let left_tree = node.borrow_mut().left.take();
                let right_leaf = node.borrow_mut().right.take();
                node.borrow_mut().left = left;
                node.borrow_mut().right = right;
                preorder(left_tree, right_leaf, Some(node))
            } else {
                right
            }
        }

        preorder(root, None, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_find_peak_element() {
        assert_eq!(Solution::upside_down_binary_tree(None), None);
        assert_eq!(
            Solution::upside_down_binary_tree(linked_tree(1, None, None)),
            linked_tree(1, None, None)
        );
        assert_eq!(
            Solution::upside_down_binary_tree(linked_tree(
                1,
                linked_tree(2, linked_tree(4, None, None), linked_tree(5, None, None)),
                linked_tree(3, None, None)
            )),
            linked_tree(
                4,
                linked_tree(5, None, None),
                linked_tree(2, linked_tree(3, None, None), linked_tree(1, None, None))
            )
        );
    }
}
