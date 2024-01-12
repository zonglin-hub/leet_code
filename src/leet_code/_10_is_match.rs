//! 正则表达式匹配

use super::Solution;

impl Solution {
    /// 动态规划 (力扣官方题解)
    pub fn is_match_10(s: String, p: String) -> bool {
        let (s, p) = (s.chars().collect::<Vec<char>>(), p.chars().collect::<Vec<char>>());

        let (m, n) = (s.len(), p.len());
        let mut f = vec![vec![false; n + 1]; m + 1];
        f[0][0] = true;

        let matches = |i, j| -> bool { i != 0 && (p[j - 1] == '.' || s[i - 1] == p[j - 1]) };
        for i in 0..=m {
            for j in 1..=n {
                f[i][j] = if p[j - 1] == '*' {
                    matches(i, j - 1) && f[i - 1][j] || f[i][j - 2]
                } else {
                    matches(i, j) && f[i - 1][j - 1]
                }
            }
        }

        f[m][n]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_match_10() {
        assert!(Solution::is_match_10("aa".to_string(), "a*".to_string()),);
        assert!(Solution::is_match_10("ab".to_string(), ".*".to_string()),);
        assert!(!Solution::is_match_10("aa".to_string(), "a".to_string()));
    }
}
