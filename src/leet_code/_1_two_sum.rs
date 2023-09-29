//! 基数之和 1 | 15 | 16 | 18

use super::Solution;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::ControlFlow;

/// 两数之和
impl Solution {
    /// 使用哈希表来查找两个数之和等于目标值的方法
    pub fn two_sum_1_v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, k) in nums.iter().enumerate() {
            if let Some(&a) = map.get(&(target - k)) {
                return vec![a, i.try_into().expect("")];
            }
            map.insert(k, i.try_into().expect(""));
        }

        vec![]
    }

    pub fn two_sum_1_v4(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        let r = nums.iter().enumerate().try_for_each(|(i, &k)| {
            let i = i.try_into().expect("");
            if let Some(&a) = map.get(&(target - k)) {
                return ControlFlow::Break(vec![a, i]);
            }
            map.insert(k, i);
            ControlFlow::Continue(())
        });

        match r {
            ControlFlow::Break(v) => v,
            ControlFlow::Continue(()) => vec![],
        }
    }

    /// 暴力枚举
    pub fn two_sum_1_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
    pub fn two_sum_1_v3(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

/// 三数之和
impl Solution {
    pub fn three_sum_15_v1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        nums.sort();
        let mut result = vec![];

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[l], nums[r]]);

                        l += 1;

                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }

                        r -= 1;

                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                }
            }
        }

        result
    }
}

/// 最接近的三数之和
impl Solution {
    /// 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。返回这三个数的和。
    ///
    /// # Approach
    ///
    /// - 以非降序对输入数组进行排序。
    /// - 对于数组中的每个元素，执行两个指针搜索以找到总和最接近目标-当前元素的另外两个元素。
    /// - 如果当前总和正好等于目标，立即返回。
    ///
    /// Time complexity: O(n^2)\
    /// Space complexity: O(1)
    ///
    /// # 优化 替换 sort | sort_unstable
    /// sort函数是稳定排序算法，它保证相等的元素之间的顺序不会改变。而sort_unstable函数则是非稳定排序算法，它不保证相等的元素之间的顺序不会改变。
    /// 在这段代码中，我们并不关心相等元素之间的顺序，只关心排序后的元素是否满足我们的目标，排序的顺序不影响结果的正确性。
    /// 在相同时间复杂度下，非稳定排序算法比稳定排序算法获得更好的性能，因此使用sort_unstable函数可以进一步提高代码的性能。
    pub fn three_sum_closest_16_v1(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let (mut ans, mut diff) = (i32::MAX, i32::MAX);
        for i in 0..n {
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                let new_diff = (sum - target).abs();
                if new_diff < diff {
                    diff = new_diff;
                    ans = sum;
                }
                match sum.cmp(&target) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => return sum,
                }
            }
        }
        ans
    }
}

/// 四数之和
impl Solution {
    pub fn four_sum_18_v1(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::k_sum_v1(4, nums, target)
    }

    pub fn k_sum_v1(k: i32, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < k as usize {
            return Vec::new();
        }

        let mut results = Vec::new();
        let mut nums = nums;
        nums.sort();

        if k == 2 {
            let mut left = 0;
            let mut right = nums.len() - 1;

            while left < right {
                match (nums[left] + nums[right]).cmp(&target) {
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                    Ordering::Equal => {
                        results.push(vec![nums[left], nums[right]]);
                        left += 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                    }
                }
            }
        } else {
            for i in 0..nums.len() - k as usize + 1 {
                if nums[i] * k > target {
                    break;
                }

                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }

                let sub_results = Self::k_sum_v1(k - 1, nums[i + 1..].to_vec(), target - nums[i]);

                for mut r in sub_results {
                    r.push(nums[i]);
                    results.push(r);
                }
            }
        }

        results
    }
}
