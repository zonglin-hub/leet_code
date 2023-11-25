use super::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<char>>();
        s.push('0');
        let mut dp = vec![0; s.len()];
        dp[0] = 1;

        for i in 1..s.len() {
            if s[i - 1] != '0' {
                dp[i] += dp[i - 1]
            }
            if i > 1
                && s[i - 2] != '0'
                && (s[i - 2] as i32 - '0' as i32) * 10 + (s[i - 1] as i32 - '0' as i32) <= 26
            {
                dp[i] += dp[i - 2];
            }
        }

        dp[s.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_num_decodings() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }
}
