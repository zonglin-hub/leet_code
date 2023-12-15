//! 最长回文子串

use super::Solution;

impl Solution {
    /// 动态规划 (力扣官方题解)
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![true; n]; n];
        let mut res = (0, 0);

        for k in 1..n {
            for i in 0..(n - k) {
                if k == 1 {
                    dp[i][i + k] = s[i] == s[i + 1];
                } else {
                    dp[i][i + k] = s[i] == s[i + k] && dp[i + 1][i + k - 1];
                }

                if dp[i][i + k] {
                    res = (i, i + k);
                }
            }
        }

        s[res.0..=res.1].iter().collect()
    }

    /// 窗口移动法
    pub fn longest_palindrome_5_v1(s: String) -> String {
        fn is_palindrome(v: &[char]) -> bool {
            for i in 0..v.len() / 2 {
                if v[i] != v[v.len() - 1 - i] {
                    return false;
                }
            }
            true
        }
        let sv = s.chars().collect::<Vec<char>>();
        let mut windows = s.len();
        let mut head = 0;
        while head != sv.len() {
            if head + windows > sv.len() {
                windows -= 1;
                head = 0;
                continue;
            }
            if is_palindrome(&sv[head..head + windows]) {
                return sv[head..head + windows].iter().collect::<String>();
            }
            head += 1
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(
            Solution::longest_palindrome("babde".to_owned()),
            String::from("bab")
        );

        assert_eq!(
            Solution::longest_palindrome("cbbd".to_owned()),
            "bb".to_owned()
        );
    }
}
