use super::Solution;

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut dp = vec![vec![0; n]; n];
        let sum = stones
            .iter()
            .scan(0, |state, &x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<i32>>();

        (0..n - 1).for_each(|i| {
            dp[i][i + 1] = std::cmp::max(stones[i], stones[i + 1]);
        });

        (2..n).for_each(|s| {
            (0..n - s).for_each(|i| {
                dp[i][i + s] = std::cmp::max(
                    sum[i + s] - sum[i + 1] + stones[i + 1] - dp[i + 1][i + s],
                    sum[i + s - 1] - sum[i] + stones[i] - dp[i][i + s - 1],
                )
            })
        });

        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::stone_game_vii(vec![5, 3, 1, 4, 2]), 6);
        assert_eq!(Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2]), 122);
    }
}
