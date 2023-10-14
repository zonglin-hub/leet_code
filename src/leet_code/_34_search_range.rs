//! 在排序数组中查找元素的第一个和最后一个位置

use super::Solution;

impl Solution {
    /// 二分
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.partition_point(|&x| x < target) as i32;
        let r = nums.partition_point(|&x| x <= target) as i32;
        match l < r {
            true => vec![l, r - 1],
            false => vec![-1, -1],
        }
    }
}
