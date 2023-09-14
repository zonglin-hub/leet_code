//! 杨辉三角
use crate::Solution;

impl Solution {
    /// f(i)(j) = f(i-1)(j-1) + f(i-1)(j)
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
    }
}
