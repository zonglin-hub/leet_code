//! 打家劫舍 III

use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn rob_337(root: TreeNodePtr) -> i32 {
        // 计算root节点的最大值
        Self::rob(&root)[1]
    }

    pub fn rob(root: &TreeNodePtr) -> [i32; 2] {
        // 如果root节点存在，则计算root节点的左右子节点的最大值
        if let Some(n) = root {
            // 计算左子节点的最大值
            let dp_l = Self::rob(&n.borrow().left);
            // 计算右子节点的最大值
            let dp_r = Self::rob(&n.borrow().right);
            [
                // 左右子节点的最大值
                dp_l[1] + dp_r[1],
                // 左右子节点的最大值的最大值
                (dp_l[0] + dp_r[0] + n.borrow().val).max(dp_l[1] + dp_r[1]),
            ]
        } else {
            // 如果root节点不存在，则返回0
            [0; 2]
        }
    }
}
