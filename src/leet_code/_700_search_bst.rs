use super::{Solution, TreeNodePtr};
// use std::cmp::Ordering;

impl Solution {
    // pub fn _search_bst(root: TreeNodePtr, val: i32) -> TreeNodePtr {
    //     match root.is_some() {
    //         true => {
    //             let v = root.as_ref()?.borrow().val;
    //             let mut tree = root.as_ref()?.borrow_mut();
    //             match v.cmp(&val) {
    //                 Ordering::Less => Self::search_bst(tree.right.take(), val),
    //                 Ordering::Greater => Self::search_bst(tree.left.take(), val),
    //                 Ordering::Equal => root.clone(),
    //             }
    //         }
    //         false => root,
    //     }
    // }

    pub fn search_bst(root: TreeNodePtr, val: i32) -> TreeNodePtr {
        let mut cur = root;
        while let Some(node) = cur {
            let x = node.borrow().val;
            match x {
                x if val < x => cur = node.borrow_mut().left.take(),
                x if val > x => cur = node.borrow_mut().right.take(),
                _ => return Some(node),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::linked_tree;
    use crate::leet_code::Solution;

    #[test]
    fn test_generate_trees() {
        let root = linked_tree(
            4,
            linked_tree(2, linked_tree(1, None, None), linked_tree(3, None, None)),
            linked_tree(7, None, None),
        );

        assert_eq!(Solution::search_bst(root.clone(), 5), None);
        assert_eq!(
            Solution::search_bst(root.clone(), 2),
            linked_tree(2, linked_tree(1, None, None), linked_tree(3, None, None))
        );
    }
}
