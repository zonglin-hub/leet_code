//! 盛最多水的容器

use super::Solution;

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
    use super::Solution;

    #[test]
    fn test_max_area_v1() {
        assert_eq!(Solution::max_area_v1(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area_v1(vec![1, 1]), 1);
    }
}
