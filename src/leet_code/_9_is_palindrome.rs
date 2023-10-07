//! 回文数

use super::Solution;

impl Solution {
    pub fn is_palindrome_v2(x: i32) -> bool {
        x.to_string().chars().rev().eq(x.to_string().chars())
    }
}
