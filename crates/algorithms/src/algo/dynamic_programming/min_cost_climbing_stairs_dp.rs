#![allow(unused)]

use std::cmp;

/// 爬楼梯最小代价：动态规划
fn min_cost_climbing_stairs_dp(cost: &[i32]) -> i32 {
    let n = cost.len() - 1;
    if n == 1 || n == 2 {
        return cost[n];
    }
    // 初始化 dp 表，用于存储子问题的解
    let mut dp = vec![-1; n + 1];
    // 初始状态：预设最小子问题的解
    dp[1] = cost[1];
    dp[2] = cost[2];
    // 状态转移：从较小子问题逐步求解较大子问题
    for i in 3..=n {
        dp[i] = cmp::min(dp[i - 1], dp[i - 2]) + cost[i];
    }
    dp[n]
}

#[test]
fn test_min_cost_climbing_stairs_dp() {
    let cost = vec![0, 1, 10, 1];
    assert_eq!(min_cost_climbing_stairs_dp(&cost), 2);
}
