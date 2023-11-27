//! 整数反转

use super::Solution;

impl Solution {
    pub fn reverse_7_v1(x: i32) -> i32 {
        x.abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
            * x.signum()
    }
}

#[cfg(test)]
mod tests {

    use crate::leet_code::Solution;

    #[test]
    fn test_reverse_7_v1() {
        assert_eq!(Solution::reverse_7_v1(123), 321);
        assert_eq!(Solution::reverse_7_v1(-123), -321);
        assert_eq!(Solution::reverse_7_v1(120), 21);
        assert_eq!(Solution::reverse_7_v1(0), 0);
    }
}
