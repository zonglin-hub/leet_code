//! 正则表达式匹配
//!

use crate::Solution;

impl Solution {
    pub fn is_match_v1(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let match_c = |i, j| -> bool { i != 0 && (p[j - 1] == '.' || s[i - 1] == p[j - 1]) };
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        (0..=s.len()).for_each(|i| {
            (1..=p.len()).for_each(|j| {
                dp[i][j] = if p[j - 1] == '*' {
                    match_c(i, j - 1) && dp[i - 1][j] || dp[i][j - 2]
                } else {
                    match_c(i, j) && dp[i - 1][j - 1]
                }
            })
        });
        dp[s.len()][p.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match_v1() {
        /*
            输入：s = "aa", p = "a"
            输出：false
            解释："a" 无法匹配 "aa" 整个字符串。
        */
        assert_eq!(
            Solution::is_match_v1("aa".to_string(), "a".to_string()),
            false
        );

        /*
            输入：s = "aa", p = "a*"
            输出：true
            解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
        */
        assert_eq!(
            Solution::is_match_v1("aa".to_string(), "a*".to_string()),
            true
        );

        /*
            输入：s = "ab", p = ".*"
            输出：true
            解释：".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
        */
        assert_eq!(
            Solution::is_match_v1("ab".to_string(), ".*".to_string()),
            true
        );
    }
}
