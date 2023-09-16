//! 两数之和
//!
//! 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
//!
//! 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
//!
//! 你可以按任意顺序返回答案。
//!
//! 示例 1：
//!
//! > 输入：`nums = [2,7,11,15], target = 9` <br>
//! > 输出：`[0,1]` <br>
//! > 解释：因为 `nums[0] + nums[1] == 9` ，返回 `[0, 1]` 。
//!
//! 示例 2：
//!
//! > 输入：`nums = [3,2,4], target = 6` <br>
//! > 输出：`[1,2]`
//!
//! 示例 3：
//!
//! > 输入：`nums = [3,3], target = 6` <br>
//! > 输出：`[0,1]`
//!
//! 提示：
//!
//! - `2 <= nums.length <= 104`
//! - `-109 <= nums[i] <= 109`
//! - `-109 <= target <= 109
//! - **只会存在一个有效答案**

use std::collections::HashMap;

use super::Solution;

// use super::Solution;

impl Solution {
    #[doc = "哈希表"]
    pub fn two_sum_v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 如果数组长度为1，并且数组中的元素和目标值相等，则返回空的数组
        if nums.len() == 1 && nums[0] == target {
            return vec![0];
        }
        // 创建一个HashMap，用于存储数组中的元素和元素的索引
        let mut map = HashMap::new();
        // 遍历数组，依次判断每个数和目标数的差是否已经存在于哈希表中，如果存在，则找到了符合条件的两个数，返回它们的下标即可。
        for (k, v) in nums.into_iter().enumerate() {
            // 如果map中存在target - v的值，则返回元素和索引
            if let Some(&a) = map.get(&(target - v)) {
                return vec![a, k as i32];
            }
            // 如果差不存在于哈希表中，则将当前数及其下标插入哈希表中，等待后续的查找。
            map.insert(v, k as i32);
        }
        // 如果遍历完整个数组都没有找到符合条件的两个数，则返回空数组。
        vec![]
    }
}

impl Solution {
    /// 暴力枚举
    pub fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 如果数组长度为1，并且数组中的元素和目标值相等，则返回空的数组
        if nums.len() == 1 && nums[0] == target {
            return vec![0];
        }

        /*
            [2, 7, 11, 15]

            2 + 7
            2 + 11
            2 + 15

            7 + 11
            7 + 15

            11 + 15
        */
        // 遍历数组，查找目标值target在数组中的位置
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                // 如果目标值target在数组中的位置与i和j相等，则返回结果
                if target == nums[i] + nums[j] {
                    return vec![i as i32, j as i32];
                }
            }
        }
        // 如果没有找到，则返回空的数组
        vec![]
    }
}

impl Solution {
    /// 双指针
    pub fn two_sum_v3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 如果数组长度为1，并且数组中的元素和目标值相等，则返回空的数组
        if nums.len() == 1 && nums[0] == target {
            return vec![0];
        }

        let len = nums.len();

        for i in 0..len {
            let left = i;
            let mut right = len - 1;
            while left < right {
                if nums[left] + nums[right] == target {
                    return vec![i as i32, right as i32];
                }
                right -= 1;
            }
        }
        vec![]
    }
}
