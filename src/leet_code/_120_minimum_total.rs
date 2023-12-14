use super::Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; triangle.len() + 1];

        for row in triangle.into_iter().rev() {
            for i in 0..row.len() {
                dp[i] = row[i] + dp[i].min(dp[i + 1]);
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_total() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }
}
