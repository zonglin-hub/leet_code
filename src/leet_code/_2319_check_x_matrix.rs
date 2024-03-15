use super::Solution;

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        for i in 0..n {
            for j in 0..n {
                if (i == j || i + j + 1 == n) && grid[i][j] == 0 {
                    return false;
                }
                if i != j && i + j + 1 != n && grid[i][j] != 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert!(Solution::check_x_matrix(vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2]
        ]));
        assert!(!Solution::check_x_matrix(vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]]));
    }
}
