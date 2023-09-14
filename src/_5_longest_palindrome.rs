//! 最长回文子串
//!
//! 给你一个字符串 s，找到 s 中最长的回文子串。
//!
//! 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。

use crate::Solution;

impl Solution {
    /// 给你一个字符串 s，找到 s 中最长的回文子串。
    /// 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。
    pub fn longest_palindrome_v1(s: String) -> String {
        let sv = s.chars().collect::<Vec<char>>();
        let mut windows = s.len();
        let mut head = 0;
        while head != sv.len() {
            if head + windows > sv.len() {
                windows -= 1;
                head = 0;
                continue;
            }
            if Self::is_palindrome(&sv[head..head + windows]) {
                return sv[head..head + windows].iter().collect::<String>();
            }
            head += 1
        }
        "".to_string()
    }

    fn is_palindrome(v: &[char]) -> bool {
        for i in 0..v.len() / 2 {
            if v[i] != v[v.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome_v1() {
        /*
            输入：s = "babad"
            输出："bab"
            解释："aba" 同样是符合题意的答案。
        */
        assert_eq!(
            Solution::longest_palindrome_v1("babad".to_owned()),
            "bab".to_owned()
        );

        /*
            输入：s = "cbbd"
            输出："bb"
        */
        assert_eq!(
            Solution::longest_palindrome_v1("cbbd".to_owned()),
            "bb".to_owned()
        );
    }
}
