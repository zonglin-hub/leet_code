#![allow(unused)]
pub struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/two-sum/
    ///
    /// 两数之和
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 1 && nums[0] == target {
            return vec![0];
        }
        let mut map = std::collections::HashMap::new();
        for (k, v) in nums.into_iter().enumerate() {
            if let Some(&a) = map.get(&(target - v)) {
                return vec![a, k as i32];
            }
            map.insert(v, k as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![], 9), vec![]);
        assert_eq!(Solution::two_sum(vec![9], 9), vec![0]);
        assert_eq!(Solution::two_sum(vec![8], 9), vec![]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
