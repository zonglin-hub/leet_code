use super::Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let (ring, key) = (ring.as_bytes(), key.as_bytes());
        let n = ring.len();
        let m = key.len();
        let mut dp = vec![vec![usize::MAX; n]; m + 1];
        dp[m] = vec![0; n];

        for (i, &key_val) in key.iter().enumerate().rev() {
            for (j, _) in ring.iter().enumerate() {
                for (k, &ring_val) in ring.iter().enumerate() {
                    if ring_val == key_val {
                        let diff = if j < k { k - j } else { j - k };
                        dp[i][j] = dp[i][j].min(dp[i + 1][k] + diff.min(n - diff));
                    }
                }
            }
        }
        (dp[0][0] + m) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::find_rotate_steps("godding".to_owned(), "gd".to_owned()), 4);
        assert_eq!(Solution::find_rotate_steps("godding".to_owned(), "godding".to_owned()), 13);
    }
}
