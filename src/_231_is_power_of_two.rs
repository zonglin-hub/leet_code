#![allow(unused)]
struct Solution;

impl Solution {
    // 2 的幂
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }

    pub fn is_power_of_two_v1(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        if n == 0 || (n & 1) != 0 {
            return false;
        }
        Self::is_power_of_two(n / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        // 输入：n = 1
        // 输出：true
        // 解释：20 = 1
        assert_eq!(Solution::is_power_of_two_v1(1), true);

        // 输入：n = 16
        // 输出：true
        // 解释：24 = 16
        assert_eq!(Solution::is_power_of_two_v1(16), true);

        // 输入：n = 3
        // 输出：false
        assert_eq!(Solution::is_power_of_two_v1(3), false);

        // 输入：n = 4
        // 输出：true
        assert_eq!(Solution::is_power_of_two_v1(4), true);

        // 输入：n = 5
        // 输出：false
        assert_eq!(Solution::is_power_of_two_v1(5), false);
    }
}
