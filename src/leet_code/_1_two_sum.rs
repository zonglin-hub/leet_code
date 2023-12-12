//! 两数之和

use super::Solution;
use std::collections::HashMap;
// use std::ops::ControlFlow;

impl Solution {
    /// 哈希表
    ///
    /// 这个函数是用于解决 "Two Sum" 这种问题的其中一个版本。给定一个整数数组 `nums` 和一个目标值 `target`，找出数组中和为目标值的那两个整数，并返回它们的数组下标。
    ///
    /// 函数的参数包括一个整数向量 `nums` 和一个目标整数 `target`。函数返回一个向量，包含两个整数的索引。
    ///
    /// 函数的实现采用哈希表（HashMap）作为辅助数据结构。哈希表存储的是每个元素值与其索引的映射关系。
    ///
    /// 下面是函数的详细步骤：
    ///
    /// 1. 初始化一个空的哈希表 `map`。
    /// 2. 遍历输入数组 `nums`。对于每个元素 `k`，同时获取其索引 `i`。
    /// 3. 检查 `target - k` 是否在哈希表 `map` 中。如果是，说明找到了两个数，它们的和等于 `target`。返回这两个数的索引，即 `map` 中存储的索引值和当前的索引 `i`。注意，由于 `map` 中存储的是 `i32` 类型的索引值，因此需要使用 `try_into()` 方法将其转换为 `usize` 类型。
    /// 4. 如果 `target - k` 不在哈希表中，则将当前的元素值 `k` 和索引 `i` 存入哈希表中，然后继续遍历下一个元素。
    /// 5. 如果遍历完整个数组都没有找到满足条件的两个数，那么返回一个空向量。
    ///
    /// 这种解法的时间复杂度为 O(n)，其中 n 是输入数组的长度，因为我们只遍历了一次数组。空间复杂度也是 O(n)，因为在最坏的情况下，我们可能需要存储数组中的每个元素到哈希表中。
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashtable = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(&a) = hashtable.get(&(target - num)) {
                return vec![a, i as i32];
            }

            hashtable.insert(num, i as i32);
        }

        vec![]
    }

    // pub fn two_sum_1_v4(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut map = HashMap::new();

    //     let r = nums.iter().enumerate().try_for_each(|(i, &k)| {
    //         let i = i.try_into().unwrap();
    //         if let Some(&a) = map.get(&(target - k)) {
    //             return ControlFlow::Break(vec![a, i]);
    //         }
    //         map.insert(k, i);
    //         ControlFlow::Continue(())
    //     });

    //     match r {
    //         ControlFlow::Break(v) => v,
    //         ControlFlow::Continue(()) => vec![],
    //     }
    // }

    // pub fn two_sum_1_v5(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     nums.iter()
    //         .enumerate()
    //         .try_fold(HashMap::new(), |mut map, (i, n)| {
    //             map.get(&(target - n))
    //                 .and_then(|&v| match v {
    //                     v if v != i => Err(vec![v as i32, i as i32]).into(),
    //                     _ => Ok(()).into(),
    //                 })
    //                 .unwrap_or(Ok(()))?;
    //             map.insert(n, i);
    //             Ok(map)
    //         })
    //         .err()
    //         .unwrap_or(vec![])
    // }

    // /// 暴力枚举
    // pub fn two_sum_1_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     if nums.len() == 1 && nums[0] == target {
    //         return vec![0];
    //     }

    //     for i in 0..nums.len() {
    //         for j in i + 1..nums.len() {
    //             if target == nums[i] + nums[j] {
    //                 return vec![i as i32, j as i32];
    //             }
    //         }
    //     }
    //     vec![]
    // }

    // pub fn two_sum_1_v3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     if nums.len() == 1 && nums[0] == target {
    //         return vec![0];
    //     }
    //     let len = nums.len();
    //     for i in 0..len {
    //         let left = i;
    //         let mut right = len - 1;
    //         while left < right {
    //             if nums[left] + nums[right] == target {
    //                 return vec![i as i32, right as i32];
    //             }
    //             right -= 1;
    //         }
    //     }
    //     vec![]
    // }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
