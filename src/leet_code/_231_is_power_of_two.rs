//! 2 的幂

use super::Solution;

impl Solution {
    pub fn is_power_of_two_231(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }

    pub fn is_power_of_two_v1(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        if n == 0 || (n & 1) != 0 {
            return false;
        }
        Self::is_power_of_two_231(n / 2)
    }
}
