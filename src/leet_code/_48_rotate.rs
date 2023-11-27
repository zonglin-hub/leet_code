use super::Solution;

impl Solution {
    /// 这个函数名为 `rotate`，它接收一个可变的二维向量（或者说一个矩阵）作为参数，并且没有返回值（返回类型为 `()`）。这个函数的目的是原地旋转这个矩阵。
    ///
    /// 具体来说，这个函数做了以下事情：
    ///
    /// 1. **交换矩阵的对称位置**：对于矩阵中的每一个元素，它交换了矩阵的对称位置。例如，如果有一个元素 `matrix[i][j]`，它会交换这个元素与 `matrix[j][i]` 的位置。这确保了每一行的最右边的元素都移动到了该行的最左边，而每一列的最下边的元素都移动到了该列的最上边。
    /// 2. **反转每一行**：在完成第一步后，每一行的元素需要反转。这确保了每一列的最右边的元素都移动到了该列的最左边，每一行最下边的元素都移动到了该行的最上边。
    ///
    /// 所以，如果你想象这个矩阵是一个二维的表格，这个函数的作用就是旋转这个表格90度。
    ///
    /// 注意：这个函数假设输入的矩阵是方阵（即行数和列数相等）。如果输入的矩阵不是方阵，这个函数可能会产生错误的结果或导致未定义的行为。
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() - 1 {
            for j in i + 1..matrix.len() {
                let t = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = t;
            }
        }
        matrix.iter_mut().for_each(|v| v.reverse())
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_rotate() {
        let mut t_vec1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut t_vec1);
        assert_eq!(t_vec1, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

        let mut t_vec2 = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut t_vec2);
        assert_eq!(
            t_vec2,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
