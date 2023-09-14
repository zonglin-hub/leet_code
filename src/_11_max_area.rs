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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area_v1() {
        /*
            输入：[1,8,6,2,5,4,8,3,7]
            输出：49
            解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
        */
        assert_eq!(Solution::max_area_v1(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);

        /*
            输入：height = [1,1]
            输出：1
        */
        assert_eq!(Solution::max_area_v1(vec![1, 1]), 1);
    }
}
