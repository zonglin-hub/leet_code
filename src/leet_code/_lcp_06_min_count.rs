//! LCP 06. 拿硬币

use super::Solution;

impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        coins.iter().fold(0, |sum, &x| (x + 1) / 2 + sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_count() {
        assert_eq!(Solution::min_count(vec![4, 2, 1]), 4);
        assert_eq!(Solution::min_count(vec![2, 3, 10]), 8);
    }
}
