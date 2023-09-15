//! 合并二叉树

use crate::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn merge_trees_v1(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (None, r) | (r, None) => r,
            // (Some(r1), Some(r2)) => match (r1.borrow_mut(), r2.borrow_mut()) {
            //     (mut r1, mut r2) => Some(Rc::new(RefCell::new(TreeNode {
            //         val: r1.val + r2.val,
            //         left: Self::merge_trees(r1.left.take(), r2.left.take()),
            //         right: Self::merge_trees(r1.right.take(), r2.right.take()),
            //     }))),
            // },
            (Some(r1), Some(r2)) => {
                let mut r1 = r1.borrow_mut();
                let mut r2 = r2.borrow_mut();
                let mut new_val = TreeNode::new(r1.val + r2.val);
                new_val.left = Self::merge_trees_v1(r1.left.take(), r2.left.take());
                new_val.right = Self::merge_trees_v1(r1.right.take(), r2.right.take());
                Some(Rc::new(RefCell::new(new_val)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::create_tree_node;

    use super::*;

    #[test]
    fn test_merge_trees_v1() {
        /*
            输入：root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
            输出：[3,4,5,5,4,null,7]
        */
        assert_eq!(
            Solution::merge_trees_v1(
                create_tree_node(
                    1,
                    create_tree_node(3, create_tree_node(5, None, None), None),
                    create_tree_node(2, None, None),
                ),
                create_tree_node(
                    2,
                    create_tree_node(1, None, create_tree_node(4, None, None)),
                    create_tree_node(3, None, create_tree_node(7, None, None)),
                )
            ),
            create_tree_node(
                3,
                create_tree_node(
                    4,
                    create_tree_node(5, None, None),
                    create_tree_node(4, None, None),
                ),
                create_tree_node(5, None, create_tree_node(7, None, None)),
            )
        );

        /*
            输入：root1 = [1], root2 = [1,2]
            输出：[2,2]
        */
        assert_eq!(
            Solution::merge_trees_v1(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
            ),
            create_tree_node(2, None, create_tree_node(2, None, None))
        );
    }
}
