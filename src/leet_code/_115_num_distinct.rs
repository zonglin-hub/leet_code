use super::Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let t = t.chars().collect::<Vec<char>>();
        let mut dp = vec![0; t.len() + 1];
        dp[0] = 1;

        for c in s.chars() {
            for i in (1..=t.len()).rev() {
                dp[i] += if c == t[i - 1] { dp[i - 1] } else { 0 }
            }
        }

        dp[t.len()]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_num_distinct() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
            3
        );
        assert_eq!(
            Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
            5
        );
    }
}
