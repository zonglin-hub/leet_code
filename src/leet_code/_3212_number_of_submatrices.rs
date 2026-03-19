use crate::leet_code::Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut ans = 0;
        let mut prefix = vec![vec![vec![0; 2]; cols + 1]; rows + 1];

        for i in 0..rows {
            for j in 0..cols {
                match grid[i][j] {
                    'X' => {
                        prefix[i + 1][j + 1][0] =
                            prefix[i + 1][j][0] + prefix[i][j + 1][0] - prefix[i][j][0] + 1;
                        prefix[i + 1][j + 1][1] = 1;
                    }
                    'Y' => {
                        prefix[i + 1][j + 1][0] =
                            prefix[i + 1][j][0] + prefix[i][j + 1][0] - prefix[i][j][0] - 1;
                        prefix[i + 1][j + 1][1] =
                            if prefix[i + 1][j][1] == 1 || prefix[i][j + 1][1] == 1 { 1 } else { 0 };
                    }
                    _ => {
                        prefix[i + 1][j + 1][0] = prefix[i + 1][j][0] + prefix[i][j + 1][0] - prefix[i][j][0];
                        prefix[i + 1][j + 1][1] =
                            if prefix[i + 1][j][1] == 1 || prefix[i][j + 1][1] == 1 { 1 } else { 0 };
                    }
                }
                if prefix[i + 1][j + 1][0] == 0 && prefix[i + 1][j + 1][1] == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_number_of_submatrices() {
        let grid = vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']];
        assert_eq!(Solution::number_of_submatrices(grid), 3);

        let grid = vec![vec!['X', 'X'], vec!['X', 'Y']];
        assert_eq!(Solution::number_of_submatrices(grid), 0);

        let grid = vec![vec!['.', '.'], vec!['.', '.']];
        assert_eq!(Solution::number_of_submatrices(grid), 0);
    }
}
