//! 正则表达式匹配
//!

use super::Solution;

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
