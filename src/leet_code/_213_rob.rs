//! 打家劫舍 Ⅱ
//!
//! 你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 围成一圈 ，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警 。
//!
//! 给定一个代表每个房屋存放金额的非负整数数组，计算你 在不触动警报装置的情况下 ，今晚能够偷窃到的最高金额。
//!
//! 示例 1：
//!
//! > 输入：`nums = [2,3,2]` <br>
//! > 输出：3 <br>
//! > 解释：你不能先偷窃 1 号房屋（金额 = 2），然后偷窃 3 号房屋（金额 = 2）, 因为他们是相邻的。
//!
//! 示例 2：
//!
//! > 输入：`nums = [1,2,3,1]` <br>
//! > 输出：4 <br>
//! > 解释：你可以先偷窃 1 号房屋（金额 = 1），然后偷窃 3 号房屋（金额 = 3）。偷窃到的最高金额 = 1 + 3 = 4 。
//!
//! 示例 3：
//!
//! > 输入：`nums = [1,2,3]` <br>
//! > 输出：3 <br>
//!
//! 提示：
//!
//! - `1 <= nums.length <= 100`
//! - `0 <= nums[i] <= 400`

use super::Solution;

impl Solution {
    ///
    /// 对于整个数组 nums，分别取出第一个元素到倒数第二个元素和第二个元素到最后一个元素，分别求出最大财富数。
    /// 把这两个最大财富数和第一个元素取最大值，就是整个数组的最大财富数了。
    /// 求最大财富数的方法是使用动态规划的思想，用一个元组 (a, b) 存储上一步的最大财富数和前一步的最大财富数，然后遍历数组，对于当前元素 x，更新元组 (a, b) 为 (b, max(a + x, b))，最后返回元组的第二个元素。
    pub fn rob_2_v1(nums: Vec<i32>) -> i32 {
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
    pub fn rob_2_v2(nums: Vec<i32>) -> i32 {
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