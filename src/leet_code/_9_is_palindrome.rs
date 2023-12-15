//! 回文数

use super::Solution;

impl Solution {
    /// 反转一半数字 (力扣官方题解)
    pub fn is_palindrome_9_v1(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut reverted_number = 0;
        while x > reverted_number {
            reverted_number = reverted_number * 10 + x % 10;
            x /= 10;
        }

        x == reverted_number || x == reverted_number / 10
    }

    /// 反转全部数字
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

    #[test]
    fn test_is_palindrome_9_v1() {
        assert!(Solution::is_palindrome_9_v1(121));
        assert_eq!(Solution::is_palindrome_9_v1(-121), false);
        assert_eq!(Solution::is_palindrome_9_v1(10), false);
    }
}
