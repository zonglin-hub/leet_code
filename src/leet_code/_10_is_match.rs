//! 正则表达式匹配

// use itertools::Itertools;

use super::Solution;

impl Solution {
    /// 动态规划
    pub fn is_match_v1(s: String, p: String) -> bool {
        let (s, p) = (
            s.chars().collect::<Vec<char>>(),
            p.chars().collect::<Vec<char>>(),
        );
        // let s = s.chars().collect::<Vec<char>>();
        // let p = p.chars().collect::<Vec<char>>();

        let (s_n, p_n) = (s.len(), p.len());
        // let s_n = s.len();
        // let p_n = p.len();

        let match_c = |i, j| -> bool { i != 0 && (p[j - 1] == '.' || s[i - 1] == p[j - 1]) };

        let mut dp = vec![vec![false; p_n + 1]; s_n + 1];
        dp[0][0] = true;

        for i in 0..=s.len() {
            for j in 1..=p.len() {
                // dp[i][j] = match p[j - 1] {
                //     '*' => match_c(i, j - 1) && dp[i - 1][j] || dp[i][j - 2],
                //     _ => match_c(i, j) && dp[i - 1][j - 1],
                // }
                dp[i][j] = if p[j - 1] == '*' {
                    match_c(i, j - 1) && dp[i - 1][j] || dp[i][j - 2]
                } else {
                    match_c(i, j) && dp[i - 1][j - 1]
                }
            }
        }

        dp[s_n][p_n]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_match_v1() {
        assert_eq!(
            Solution::is_match_v1("aa".to_string(), "a".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match_v1("aa".to_string(), "a*".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match_v1("ab".to_string(), ".*".to_string()),
            true
        );
    }
}
