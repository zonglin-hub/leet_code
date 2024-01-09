use super::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let set = dictionary.into_iter().collect::<HashSet<_>>();
        let mut dp = vec![0; s.len() + 1];
        for i in 1..=s.len() {
            dp[i] = dp[i - 1] + 1;
            for j in (1..=i).rev() {
                if set.contains(&s[j - 1..i]) {
                    dp[i] = dp[i].min(dp[j - 1]);
                }
            }
        }
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_extra_char() {
        assert_eq!(
            Solution::min_extra_char(
                "leetscode".to_owned(),
                vec!["leet".to_owned(), "code".to_owned(), "leetcode".to_owned()]
            ),
            1
        );
        assert_eq!(
            Solution::min_extra_char(
                "sayhelloworld".to_owned(),
                vec!["hello".to_owned(), "world".to_owned()]
            ),
            3
        );
    }
}
