use crate::leet_code::Solution;

impl Solution {
    /// 给定一个 m x n 的二进制矩阵 mat，返回矩阵 mat 中特殊位置的数量。
    /// 如果位置 (i, j) 满足 mat[i][j] == 1 并且行 i 与列 j 中的所有其他元素都是 0（行和列的下标从 0 开始计数），那么它被称为 特殊 位置。
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        if m == 0 {
            return 0;
        }

        let n = mat[0].len();

        // 矩阵：
        // [1, 0, 0]
        // [0, 0, 1]
        // [1, 0, 0]
        //
        // 图示：
        // +---+---+---+
        // | 1 | 0 | 0 |
        // +---+---+---+
        // | 0 | 0 | 1 |
        // +---+---+---+
        // | 1 | 0 | 0 |
        // +---+---+---+
        // 特殊位置：只有 (1,2) 是特殊元素（该行只有它一个1，该列也只有它一个1）
        // 检查：
        // - 第1行有两个1，不符合。
        // - 第2行只有一个1（位于列2），且列2也只有一个1，符合。
        // - 第3行有一个1，但列0有两个1（第1行和第3行），不符合。
        // 计算每行和每列的和
        let mut rows_sum = vec![0; m];
        let mut cols_sum = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                rows_sum[i] += mat[i][j];
                cols_sum[j] += mat[i][j];
            }
        }

        // 统计符合条件的元素个数
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 && rows_sum[i] == 1 && cols_sum[j] == 1 {
                    res += 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_num_special() {
        let mat_1 = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::num_special(mat_1), 3); // 所有对角线元素均为特殊

        let mat_2 = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];
        assert_eq!(Solution::num_special(mat_2), 1);
    }
}
