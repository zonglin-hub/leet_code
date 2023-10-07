//! 杨辉三角

use super::Solution;

impl Solution {
    /// f(i)(j) = f(i-1)(j-1) + f(i-1)(j)
    pub fn generate_118(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(num_rows as usize);

        // 杨辉三角的深度等于 num 的个数
        for i in 0..num_rows as usize {
            res.push(vec![1; i + 1]);
            if i != 0 && i != 1 {
                // 1..i 内循环次数
                for j in 1..i {
                    // 首尾都为一，
                    // 下一层 x 结果为上一层，相同索引的位置与前一位之和
                    // 杨辉三角
                    // 1
                    // 1 1
                    // 1 2 1
                    // 1 3 3 1
                    // 1 4 6 4 1
                    // 1 5 10 10 5 1
                    // 1 6 15 20 15 6 1
                    // 1 7 21 35 35 21 7 1
                    // 1 8 28 56 70 56 28 8 1
                    // 1 9 36 84 126 126 84 36 9 1
                    res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
                }
            }
        }
        res
    }
}

impl Solution {
    /// 定义一个长度为 rowIndex + 1 的一维数组 res
    /// 从后向前计算，每次更新 `res[j]` 即可。
    /// 具体而言，第 j 列的值更新为第 j-1 列的值加上第 j 列的值，即 `res[j] = res[j - 1] + res[j]`
    pub fn get_row_119(row_index: i32) -> Vec<i32> {
        let mut res = vec![1; (row_index + 1) as usize];
        for i in 1..=row_index as usize {
            for j in (1..i).rev() {
                res[j] += res[j - 1]
            }
        }
        res
    }
}
