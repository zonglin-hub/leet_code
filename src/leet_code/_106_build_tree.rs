use crate::leet_code::TreeNode;

use super::{Solution, TreeNodePtr};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn build_tree_106(inorder: Vec<i32>, postorder: Vec<i32>) -> TreeNodePtr {
        fn recv(
            postorder: &Vec<i32>,
            map: &HashMap<i32, usize>,
            subin: (usize, usize),
            subpost: (usize, usize),
        ) -> TreeNodePtr {
            if subpost.0 >= subpost.1 {
                return None;
            }

            let len = subin.1 - *map.get(&postorder[subpost.1 - 1])? - 1;
            Some(Rc::new(RefCell::new(TreeNode {
                val: postorder[subpost.1 - 1],
                left: recv(
                    postorder,
                    map,
                    (subin.0, subin.1 - len - 1),
                    (subpost.0, subpost.1 - len - 1),
                ),
                right: recv(
                    postorder,
                    map,
                    (subin.1 - len, subin.1),
                    (subpost.1 - len - 1, subpost.1 - 1),
                ),
            })))
        }

        recv(
            &postorder,
            &inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect(),
            (0, inorder.len()),
            (0, postorder.len()),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_build_tree() {
        assert_eq!(
            Solution::build_tree_106(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            linked_tree(
                3,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )
        );
        assert_eq!(
            Solution::build_tree_106(vec![-1], vec![-1]),
            linked_tree(-1, None, None)
        );
    }
}
