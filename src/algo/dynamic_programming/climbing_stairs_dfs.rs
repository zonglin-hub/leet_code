#![allow(unused)]

/// 搜索
fn dfs(i: usize) -> i32 {
    // 已知 dp[1] 和 dp[2] ，返回之
    if i == 1 || i == 2 {
        return i as i32;
    }
    // dp[i] = dp[i-1] + dp[i-2]
    dfs(i - 1) + dfs(i - 2)
}

/// 爬楼梯：搜索
fn climbing_stairs_dfs(n: usize) -> i32 {
    dfs(n)
}

#[test]
fn test_climbing_stairs_dfs() {
    assert_eq!(climbing_stairs_dfs(3), 3);
    assert_eq!(climbing_stairs_dfs(6), 13);
}
