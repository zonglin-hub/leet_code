use std::cell::RefCell;
use std::rc::Rc;

use crate::leet_code::TreeNode;

use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<TreeNodePtr> {
        #[inline]
        fn generate(start: i32, end: i32) -> Vec<TreeNodePtr> {
            if start > end {
                return vec![None];
            }

            let mut res = vec![];
            for i in start..=end {
                for left_tree in generate(start, i - 1) {
                    for right_tree in generate(i + 1, end) {
                        res.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: left_tree.clone(),
                            right: right_tree.clone(),
                        }))));
                    }
                }
            }
            res
        }

        if n == 0 {
            return vec![None];
        }

        generate(1, n)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::linked_tree;
    use crate::leet_code::Solution;

    #[test]
    fn test_generate_trees() {
        assert_eq!(
            Solution::generate_trees(3),
            vec![
                linked_tree(1, None, linked_tree(2, None, linked_tree(3, None, None))),
                linked_tree(1, None, linked_tree(3, linked_tree(2, None, None), None)),
                linked_tree(2, linked_tree(1, None, None), linked_tree(3, None, None)),
                linked_tree(3, linked_tree(1, None, linked_tree(2, None, None)), None),
                linked_tree(3, linked_tree(2, linked_tree(1, None, None), None), None),
            ]
        );
        assert_eq!(
            Solution::generate_trees(1),
            vec![linked_tree(1, None, None)]
        );
    }
}
