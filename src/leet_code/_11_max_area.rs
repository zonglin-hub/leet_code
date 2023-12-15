//! 盛最多水的容器

use super::Solution;

impl Solution {
    // 双指针 (力扣官方题解)
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let mut ans = 0;
        while l < r {
            ans = ans.max(height[l].min(height[r]) * (r - l) as i32);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
