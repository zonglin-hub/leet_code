//! 打家劫舍 III

use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn rob_337(root: TreeNodePtr) -> i32 {
        fn rob(root: &TreeNodePtr) -> [i32; 2] {
            if let Some(n) = root {
                let dp_l = rob(&n.borrow().left);
                let dp_r = rob(&n.borrow().right);
                [
                    dp_l[1] + dp_r[1],
                    (dp_l[0] + dp_r[0] + n.borrow().val).max(dp_l[1] + dp_r[1]),
                ]
            } else {
                [0; 2]
            }
        }

        rob(&root)[1]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{array_to_tree, Solution};

    #[test]
    fn test_rob_337() {
        assert_eq!(
            Solution::rob_337(array_to_tree(vec![
                Some(3),
                Some(2),
                Some(3),
                None,
                Some(3),
                None,
                Some(1)
            ])),
            7
        );
    }
}
