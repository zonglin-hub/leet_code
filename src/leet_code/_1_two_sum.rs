//! 两数之和

use super::Solution;
use std::collections::HashMap;
use std::ops::ControlFlow;

impl Solution {
    /// 哈希表 (力扣官方题解)
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

    /// 暴力枚举 (力扣官方题解)
    pub fn two_sum_v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if target == nums[i] + nums[j] {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    /// 不一样的编程风格
    pub fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .try_fold(HashMap::new(), |mut map, (i, n)| {
                map.get(&(target - n))
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

    /// for_each 使用控制流
    pub fn two_sum_v3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let r = nums.iter().enumerate().try_for_each(|(i, &k)| {
            let i = i.try_into().unwrap();
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

    /// 双指针
    pub fn two_sum_v4(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_v1() {
        assert_eq!(Solution::two_sum_v1(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v1(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_v1(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_v2() {
        assert_eq!(Solution::two_sum_v2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v2(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_v2(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_v3() {
        assert_eq!(Solution::two_sum_v3(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v3(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_v3(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_v4() {
        assert_eq!(Solution::two_sum_v4(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v4(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_v4(vec![3, 3], 6), vec![0, 1]);
    }
}
