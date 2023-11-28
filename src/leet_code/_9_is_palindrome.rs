//! 回文数

use super::Solution;

impl Solution {
    pub fn is_palindrome_9(x: i32) -> bool {
        x.to_string().chars().rev().eq(x.to_string().chars())
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_palindrome_9() {
        assert!(Solution::is_palindrome_9(121));
        assert_eq!(Solution::is_palindrome_9(-121), false);
        assert_eq!(Solution::is_palindrome_9(10), false);
    }
}
