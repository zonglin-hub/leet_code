//! 搜索旋转排序数组

use super::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // if let Some(i) = nums.iter().position(|&x| x == target) {
        //     i as i32
        // } else {
        //     -1
        // }
        // 等同
        match nums.iter().position(|&x| x == target) {
            Some(i) => i as i32,
            _ => -1,
        }
    }
}
