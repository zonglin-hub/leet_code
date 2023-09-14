//! 回文数

use crate::Solution;

impl Solution {
    pub fn is_palindrome_v2(x: i32) -> bool {
        x.to_string().chars().rev().eq(x.to_string().chars())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_v2() {
        /*
            输入：x = 121
            输出：true
        */
        assert!(Solution::is_palindrome_v2(121));

        /*
            输入：x = -121
            输出：false
            解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
        */
        assert_eq!(Solution::is_palindrome_v2(-121), false);

        /*
            输入：x = 10
            输出：false
            解释：从右向左读, 为 01 。因此它不是一个回文数。
        */
        assert_eq!(Solution::is_palindrome_v2(10), false);
    }
}
