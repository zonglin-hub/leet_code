//! 两数之和
//!
//! 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。

#![allow(unused)]

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// 哈希表
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

    /// 双指针
    pub fn two_sum_v3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 如果数组长度为1，并且数组中的元素和目标值相等，则返回空的数组
        if nums.len() == 1 && nums[0] == target {
            return vec![0];
        }

        let len = nums.len();

        for i in 0..len {
            let mut left = i;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_v1() {
        assert_eq!(Solution::two_sum_v1(vec![], 9), vec![]);
        assert_eq!(Solution::two_sum_v1(vec![9], 9), vec![0]);
        assert_eq!(Solution::two_sum_v1(vec![8], 9), vec![]);
        assert_eq!(Solution::two_sum_v1(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v1(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_v2() {
        assert_eq!(Solution::two_sum_v2(vec![], 9), vec![]);
        assert_eq!(Solution::two_sum_v2(vec![9], 9), vec![0]);
        assert_eq!(Solution::two_sum_v2(vec![8], 9), vec![]);
        assert_eq!(Solution::two_sum_v2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v2(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_v3() {
        assert_eq!(Solution::two_sum_v3(vec![], 9), vec![]);
        assert_eq!(Solution::two_sum_v3(vec![9], 9), vec![0]);
        assert_eq!(Solution::two_sum_v3(vec![8], 9), vec![]);
        assert_eq!(Solution::two_sum_v3(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v3(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
