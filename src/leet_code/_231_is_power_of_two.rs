//! 2 的幂

use super::Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        fn is_power(n: i32) -> bool {
            n > 0 && n & (n - 1) == 0
        }

        if n == 1 {
            return true;
        }

        if n == 0 || (n & 1) != 0 {
            return false;
        }

        is_power(n / 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_power_of_two() {
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(16), true);
        assert_eq!(Solution::is_power_of_two(3), false);
        assert_eq!(Solution::is_power_of_two(4), true);
        assert_eq!(Solution::is_power_of_two(5), false);
    }
}
