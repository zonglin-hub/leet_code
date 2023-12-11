use super::Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let columns = grid[0].len();
        if grid.is_empty() || columns == 0 {
            return 0;
        }

        let mut dp = vec![0; columns + 1];
        for i in 1..dp.len() {
            dp[i] = dp[i - 1] + grid[0][i - 1];
        }
        dp[0] = i32::MAX;

        for i in 2..=grid.len() {
            for j in 1..=columns {
                dp[j] = dp[j].min(dp[j - 1]) + grid[i - 1][j - 1];
            }
        }
        dp[columns]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
