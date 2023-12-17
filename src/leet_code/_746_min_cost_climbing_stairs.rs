use super::Solution;

impl Solution {
    pub fn min_cost_climbing_stairs_dp(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut dp = vec![1; len + 1];
        dp[0] = 0;
        dp[1] = 0;

        for i in 2..=len {
            dp[i] = std::cmp::min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2]);
        }

        dp[len]
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut prev = 0;
        let mut curr = 0;

        for i in 2..=len {
            let next = std::cmp::min(curr + cost[i - 1], prev + cost[i - 2]);
            prev = curr;
            curr = next;
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn test_min_cost_climbing_stairs_dp() {
        assert_eq!(Solution::min_cost_climbing_stairs_dp(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs_dp(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
