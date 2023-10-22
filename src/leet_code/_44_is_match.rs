use super::Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let p = p.chars().collect::<Vec<_>>();
        let slen = s.len();
        let plen = p.len();
        let mut dp = vec![vec![false; plen + 1]; slen + 1];
        dp[0][0] = true;
        for pi in 1..=plen {
            if p[pi - 1] == '*' {
                dp[0][pi] = dp[0][pi - 1]
            }
        }
        for si in 1..=slen {
            for pi in 1..=plen {
                match p[pi - 1] {
                    '*' => {
                        dp[si][pi] = dp[si][pi - 1] || dp[si - 1][pi];
                    }
                    '?' => {
                        dp[si][pi] = dp[si - 1][pi - 1];
                    }
                    ch => {
                        dp[si][pi] = ch == s[si - 1] && dp[si - 1][pi - 1];
                    }
                }
            }
        }
        dp[slen][plen]
    }
}
