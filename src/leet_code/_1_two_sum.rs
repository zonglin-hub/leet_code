//! 两数之和

use super::Solution;
use std::collections::HashMap;

impl Solution {
    /// 使用哈希表来查找两个数之和等于目标值的方法
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
        // 如果数组长度为1，并且数组中的元素为目标值，则返回一个空的结果
        if nums.len() == 1 && nums[0] == target {
            return vec![0];
        }
        // 获取数组长度
        let len = nums.len();
        // 遍历数组
        for i in 0..len {
            // 将当前元素的索引赋值给left
            let left = i;
            // 将数组长度减1，赋值给right
            let mut right = len - 1;
            // 当left小于right时，循环
            while left < right {
                // 如果left和right之间的和与目标值相等，则返回索引和right的索引
                if nums[left] + nums[right] == target {
                    return vec![i as i32, right as i32];
                }
                // 否则，right减1
                right -= 1;
            }
        }
        // 如果没有找到，则返回空的结果
        vec![]
    }
}
