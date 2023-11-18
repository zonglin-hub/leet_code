//! 搜索旋转排序数组

use super::Solution;

impl Solution {
    pub fn search_v1(nums: Vec<i32>, target: i32) -> i32 {
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

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_search_v1() {
        assert_eq!(Solution::search_v1(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search_v1(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search_v1(vec![1], 0), -1);
    }
}
