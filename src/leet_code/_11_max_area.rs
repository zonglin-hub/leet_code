//! 盛最多水的容器
use crate::Solution;

impl Solution {
    pub fn max_area_v1(height: Vec<i32>) -> i32 {
        let (mut left, mut right, mut ans) = (0, height.len() - 1, 0);
        while left < right {
            ans = ans.max(height[left].min(height[right]) * (right - left) as i32);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
    }
}
