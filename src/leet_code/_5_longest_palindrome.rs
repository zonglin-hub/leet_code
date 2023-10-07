//! 最长回文子串

use super::Solution;

impl Solution {
    /// 给你一个字符串 s，找到 s 中最长的回文子串。
    /// 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。
    pub fn longest_palindrome_5_v1(s: String) -> String {
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

impl Solution {
    fn valid(s: &[u8]) -> bool {
        let n = s.len();
        s.iter().rev().take(n).eq(s.iter().take(n))
    }

    pub fn valid_palindrome_680(s: String) -> bool {
        let s = s.into_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j {
            if s[i] != s[j] {
                /*
                    `i + 1..=j` 是一个 Rust 中的元组表示法，用来表示一个范围。在这个例子中，它表示从 `i + 1` 到 `j` 的所有整数，包括 `i + 1` 和 `j`。
                    例如，如果 `i = 0` 且 `j = 5`，那么 `0 + 1..=5` 就表示了从 `1` 到 `5` 的所有整数，即 `1, 2, 3, 4, 5`。
                */
                return Self::valid(&s[i + 1..=j]) || Self::valid(&s[i..=j - 1]);
            }

            i += 1;
            j -= 1;
        }

        true
    }
}
