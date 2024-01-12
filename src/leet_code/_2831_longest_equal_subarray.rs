//! 找出最长等值子数组
//!
//! 给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。
//! 如果子数组中所有元素都相等，则认为子数组是一个 等值子数组 。注意，空数组是 等值子数组 。
//! 从 nums 中删除最多 k 个元素后，返回可能的最长等值子数组的长度。
//! 子数组 是数组中一个连续且可能为空的元素序列。

use std::cmp;

use super::Solution;

impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut pos = vec![Vec::<i32>::new(); n + 1];

        for (i, _) in nums.iter().enumerate().take(n) {
            let x = nums[i] as usize;
            let len = pos[x].len();
            pos[x].push((i - len).try_into().unwrap());
        }

        for ps in &pos {
            let mut left = 0;

            for right in 0..ps.len() {
                while ps[right] - ps[left] > k {
                    left += 1;
                }
                ans = cmp::max(ans, right - left + 1);
            }
        }

        ans.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_longest_equal_subarray() {
        assert_eq!(Solution::longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3), 3);
        assert_eq!(Solution::longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2), 4);
    }
}
