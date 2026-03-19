use crate::leet_code::Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut ans = 0;

        // prefix[i+1][j+1][0] : 从 (0,0) 到 (i,j) 矩形内 X 与 Y 的数量差（X 贡献 +1，Y 贡献 -1）
        // prefix[i+1][j+1][1] : 该矩形内是否至少包含一个 X 或 Y（即非 '.' 字符）
        let mut prefix = vec![vec![[0, 0]; cols + 1]; rows + 1];

        for i in 0..rows {
            for j in 0..cols {
                // 计算当前单元格对差值数组的贡献
                let contribution = match grid[i][j] {
                    'X' => 1,
                    'Y' => -1,
                    _ => 0,
                };

                // 当前单元格是否为非 '.' 字符（即 X 或 Y）
                let has_current = grid[i][j] != '.';

                // 二维前缀和递推公式：
                // diff[i+1][j+1] = diff[i+1][j] + diff[i][j+1] - diff[i][j] + contribution
                // has[i+1][j+1] = has[i+1][j] || has[i][j+1] || has_current
                // 注意：这里使用 OR 运算，因为只要任何一部分包含字符，整个矩形就包含字符
                prefix[i + 1][j + 1][0] =
                    prefix[i + 1][j][0] + prefix[i][j + 1][0] - prefix[i][j][0] + contribution;
                prefix[i + 1][j + 1][1] =
                    prefix[i + 1][j][1] | prefix[i][j + 1][1] | has_current as i32;

                // 检查以 (0,0) 为左上角、当前单元格为右下角的子矩阵
                // 条件：X 与 Y 数量相等（差值为 0）且矩形内至少有一个非 '.' 字符
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
