//! 打家劫舍。

use super::Solution;

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
