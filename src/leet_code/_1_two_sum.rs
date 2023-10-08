//! 两数之和

use super::Solution;
use std::collections::HashMap;
use std::ops::ControlFlow;

impl Solution {
    /// 哈希表for)
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

    /// 哈希表(for_each)
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

    /// 哈希表(Iterator)
    pub fn two_sum_1_v5(nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .try_fold(HashMap::new(), |mut map, (i, n)| {
                map.get(&(target - n))
                    // v 是 i 下方插入值为 i
                    .and_then(|&v| match v {
                        v if v != i => Err(vec![v as i32, i as i32]).into(),
                        _ => Ok(()).into(),
                    })
                    .unwrap_or(Ok(()))?;
                map.insert(n, i);
                Ok(map)
            })
            .err()
            .unwrap_or(vec![])
    }

    /// 暴力枚举
    pub fn two_sum_1_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if target == nums[i] + nums[j] {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    /// 双指针
    pub fn two_sum_1_v3(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
