use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn is_cousins(root: TreeNodePtr, x: i32, y: i32) -> bool {
        fn find(root: &TreeNodePtr, x: i32, p: TreeNodePtr, l: i32) -> Option<(TreeNodePtr, i32)> {
            let l1 = l + 1;
            if let Some(node) = &root {
                let node = node.borrow();
                if node.val == x {
                    return Some((p, l1));
                } else {
                    if let Some(r) = find(&node.left, x, root.clone(), l1) {
                        return Some(r);
                    }
                    if let Some(r) = find(&node.right, x, root.clone(), l1) {
                        return Some(r);
                    }
                }
            }
            None
        }

        let (f1, l1) = find(&root, x, None, 0).unwrap();
        let (f2, l2) = find(&root, y, None, 0).unwrap();
        f1 != f2 && l1 == l2
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_two_sum() {
        assert!(!Solution::is_cousins(
            linked_tree(
                1,
                linked_tree(2, linked_tree(4, None, None), None),
                linked_tree(3, None, None)
            ),
            4,
            3
        ));

        assert!(Solution::is_cousins(
            linked_tree(
                1,
                linked_tree(2, None, linked_tree(4, None, None)),
                linked_tree(3, None, linked_tree(5, None, None))
            ),
            5,
            4
        ));

        assert!(!Solution::is_cousins(
            linked_tree(
                1,
                linked_tree(2, None, linked_tree(4, None, None)),
                linked_tree(3, None, None)
            ),
            2,
            3
        ));
    }
}
