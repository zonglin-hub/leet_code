#![allow(unused)]
struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/palindrome-number/
    ///
    /// 回文数
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().rev().eq(x.to_string().chars())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(121));
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
