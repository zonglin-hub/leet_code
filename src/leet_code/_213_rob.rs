//! 打家劫舍 Ⅱ

use super::Solution;

impl Solution {
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
