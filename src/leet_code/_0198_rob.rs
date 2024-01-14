//! 打家劫舍。

use super::Solution;

impl Solution {
    pub fn rob_198(nums: Vec<i32>) -> i32 {
        nums.iter().skip(1).fold((nums[0], 0), |dp, &n| (dp.0, dp.0).max((dp.1 + n, dp.0))).0
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_rob_198() {
        assert_eq!(Solution::rob_198(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob_198(vec![2, 7, 9, 3, 1]), 12);
    }
}
