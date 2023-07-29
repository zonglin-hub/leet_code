#![allow(unused)]
struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/reverse-integer/
    ///
    /// 整数反转
    pub fn reverse(x: i32) -> i32 {
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
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
    }
}
