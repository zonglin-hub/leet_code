//! 打家劫舍。
//! 相似题型：198 | 213 | 337 | 2560

use std::{cell::RefCell, rc::Rc};

use super::{Solution, TreeNode};

impl Solution {
    pub fn rob_198(nums: Vec<i32>) -> i32 {
        // 记录前一个元素的最大值和最大值的下标
        nums.iter()
            .skip(1)
            // 计算前一个元素的最大值和最大值的下标，并将其赋值给dp
            .fold((nums[0], 0), |dp, &n| {
                //
                (dp.0, dp.0).max((dp.1 + n, dp.0))
            })
            .0
    }
}

impl Solution {
    ///
    /// 对于整个数组 nums，分别取出第一个元素到倒数第二个元素和第二个元素到最后一个元素，分别求出最大财富数。
    /// 把这两个最大财富数和第一个元素取最大值，就是整个数组的最大财富数了。
    /// 求最大财富数的方法是使用动态规划的思想，用一个元组 (a, b) 存储上一步的最大财富数和前一步的最大财富数，然后遍历数组，对于当前元素 x，更新元组 (a, b) 为 (b, max(a + x, b))，最后返回元组的第二个元素。
    pub fn rob_213_v1(nums: Vec<i32>) -> i32 {
        nums[0..(nums.len() - 1)]
            .iter()
            .fold((0, 0), |(a, b), &x| (b, b.max(a + x)))
            .1
            .max(
                nums[1..nums.len()]
                    .iter()
                    .fold((0, 0), |(a, b), &x| (b, b.max(a + x)))
                    .1,
            )
            .max(nums[0])
    }
}

impl Solution {
    pub fn rob_213_v2(nums: Vec<i32>) -> i32 {
        // 计算不包含最后一个元素的最大财富
        let max1 = Self::rob_range(&nums[0..(nums.len() - 1)]);
        // 计算不包含第一个元素的最大财富
        let max2 = Self::rob_range(&nums[1..nums.len()]);
        // 计算包含第一个元素的最大财富
        let max3 = nums[0];
        // 然后，再对这三个结果取最大值
        max1.max(max2).max(max3)
    }

    /// rob_range 函数是一个辅助函数，用来计算一个区间内的最大财富。
    /// 它的实现和原函数中的 fold 操作是等价的，也就是使用动态规划的思想，用两个变量 prev_max 和 curr_max 来存储上一步的最大财富数和前一步的最大财富数，
    /// 遍历区间，对于当前元素 x，更新这两个变量为 (curr_max, curr_max.max(prev_max + x))，最后返回 curr_max 即可。
    fn rob_range(nums: &[i32]) -> i32 {
        // 上一步的最大财富数
        let mut prev_max = 0;
        // 前一步的最大财富数
        let mut curr_max = 0;
        for x in nums {
            let new_max = curr_max.max(prev_max + x);
            prev_max = curr_max;
            curr_max = new_max;
        }
        curr_max
    }
}

impl Solution {
    pub fn rob_337(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 计算root节点的最大值
        Self::rob(&root)[1]
    }

    pub fn rob(root: &Option<Rc<RefCell<TreeNode>>>) -> [i32; 2] {
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

impl Solution {
    pub fn min_capability_2560(nums: Vec<i32>, k: i32) -> i32 {
        // 获取数组中最大值
        let mut max_num = *nums.iter().max().expect("存在最大值");
        // 获取数组中最小值
        let mut min_num = *nums.iter().min().expect("存在最小值");
        // 当最小值小于最大值时，循环
        while min_num <= max_num {
            // 计算中间值
            let middle = (min_num + max_num) / 2;
            // 定义计数器
            let mut count = 0;
            // 定义是否访问过,
            let mut visited = false;

            for &x in nums.iter() {
                if x <= middle && !visited {
                    count += 1;
                    visited = true;
                } else {
                    visited = false;
                }
            }

            // 如果计数器大于等于k，则最大值为中间值减1；否则，最小值为中间值加1
            if count >= k {
                max_num = middle - 1;
            } else {
                min_num = middle + 1;
            }
        }

        // 返回最小值
        min_num
    }
}
