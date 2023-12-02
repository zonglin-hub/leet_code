use super::Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..(n + 1) {
            for j in 1..(i + 1) {
                dp[i as usize] += dp[(j - 1) as usize] * dp[(i - j) as usize]
            }
        }

        dp[n as usize]
    }

    pub fn num_trees_v1(n: i32) -> i32 {
        (0..n as u64).fold(1, |acc, n| acc * (4 * n + 2) / (n + 2)) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_num_trees() {
        assert_eq!(Solution::num_trees(3), 5);
        assert_eq!(Solution::num_trees(1), 1);
    }
}
