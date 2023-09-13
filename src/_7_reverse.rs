//! 整数反转
//!
//! 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。

use crate::types::base_type::Solution;

impl Solution {
    pub fn reverse_v1(x: i32) -> i32 {
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
    fn test_reverse_v1() {
        /*
            输入：x = 123
            输出：321
        */
        assert_eq!(Solution::reverse_v1(123), 321);

        /*
            输入：x = -123
            输出：-321
        */
        assert_eq!(Solution::reverse_v1(-123), -321);

        /*
            输入：x = 120
            输出：21
        */
        assert_eq!(Solution::reverse_v1(120), 21);

        /*
            输入：x = 0
            输出：0
        */
        assert_eq!(Solution::reverse_v1(0), 0);
    }
}
