//! 整数反转

use super::Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let str = x.abs().to_string().chars().rev().collect::<String>();
        str.parse::<i32>().unwrap_or(0) * x.signum()
    }
}

#[cfg(test)]
mod tests {

    use crate::leet_code::Solution;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }
}
