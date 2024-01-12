//! 找出字符串中第一个匹配项的下标
//!

use super::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(idx) => idx as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_str_str() {
        assert_eq!(Solution::str_str("sadbutsad".to_string(), "sad".to_string()), 0);
        assert_eq!(Solution::str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }
}
