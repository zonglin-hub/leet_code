#![allow(unused)]

/// 回溯
fn backtrack(choices: &[i32], state: i32, n: i32, res: &mut [i32]) {
    // 当爬到第 n 阶时，方案数量加 1
    if state == n {
        res[0] += 1;
    }
    // 遍历所有选择
    for &choice in choices {
        // 剪枝：不允许越过第 n 阶
        if state + choice > n {
            break;
        }
        // 尝试：做出选择，更新状态
        backtrack(choices, state + choice, n, res);
        // 回退
    }
}

/// 爬楼梯：回溯
fn climbing_stairs_backtrack(n: usize) -> i32 {
    let choices = vec![1, 2]; // 可选择向上爬 1 或 2 阶
    let state = 0; // 从第 0 阶开始爬
    let mut res = Vec::new();
    res.push(0); // 使用 res[0] 记录方案数量
    backtrack(&choices, state, n as i32, &mut res);
    res[0]
}

#[test]
fn test_climbing_stairs_backtrack() {
    assert_eq!(climbing_stairs_backtrack(3), 3);
    assert_eq!(climbing_stairs_backtrack(6), 13);
}