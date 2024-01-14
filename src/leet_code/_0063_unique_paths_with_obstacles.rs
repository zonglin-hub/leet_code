use super::Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (obstacle_grid.len(), obstacle_grid[0].len());

        let mut f = vec![0; m];
        f[0] = if obstacle_grid[0][0] == 0 { 1 } else { 0 };

        for i in 0..n {
            for j in 0..m {
                if obstacle_grid[i][j] == 1 {
                    f[j] = 0;
                    continue;
                }

                if j as i32 > 0 && obstacle_grid[i][j - 1] == 0 {
                    f[j] += f[j - 1];
                }
            }
        }
        f[m - 1]
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
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]), 1);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1]]), 0);
    }
}
