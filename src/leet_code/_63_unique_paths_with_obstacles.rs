use super::Solution;

impl Solution {
    /// 到达方格 `[i, j]` 的路径等于到达其左侧和上侧方格路径之和 `P[i][j] = P[i-1][j] + P[i][j-1]`
    /// return `P[i - 1][j - 1]`
    pub fn _unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut cache = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    cache[i][j] = 0;
                } else if i == 0 && j == 0 {
                    cache[i][j] = 1;
                } else if i == 0 {
                    cache[i][j] = cache[i][j - 1];
                } else if j == 0 {
                    cache[i][j] = cache[i - 1][j];
                } else {
                    cache[i][j] = cache[i - 1][j] + cache[i][j - 1];
                }
            }
        }
        cache[m - 1][n - 1]
    }

    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut cache = vec![0; m];
        cache[0] = if obstacle_grid[0][0] == 0 { 1 } else { 0 };

        for (i, _) in obstacle_grid.iter().enumerate().take(n) {
            for j in 0..m {
                if obstacle_grid[i][j] == 1 {
                    cache[j] = 0;
                    continue;
                }

                if j as i32 > 0 && obstacle_grid[i][j - 1] == 0 {
                    cache[j] += cache[j - 1];
                }
            }
        }
        cache[m - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_unique_paths_with_obstacles() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );

        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
    }
}
