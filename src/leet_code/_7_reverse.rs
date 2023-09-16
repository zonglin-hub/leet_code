//! 整数反转
//!
//! 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。

use super::Solution;

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
