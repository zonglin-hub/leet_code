use super::Solution;

impl Solution {
    /// 这个函数是用于找出一个由二进制字符构成的矩阵中的最大矩形面积。其中，矩阵中的每个元素只能为字符'0'或'1'。
    ///
    /// 函数的参数是一个二维字符向量 `matrix`，表示输入的二进制矩阵。
    ///
    /// 函数的主要步骤如下：
    ///
    /// 1. 首先，检查输入矩阵是否为空。如果矩阵为空或者矩阵的任意一行都为空，那么直接返回0，因为无法构成矩形。
    /// 2. 初始化变量 `rows` 和 `cols`，分别表示矩阵的行数和列数。
    /// 3. 创建一个二维向量 `heights`，并初始化为0。这个向量用于存储从当前位置向上连续的'1'的个数，也就是高度。
    /// 4. 初始化变量 `area` 为0，用于存储最大矩形的面积。
    /// 5. 遍历矩阵的每一个元素。如果当前元素为'1'，那么将 `heights` 中对应位置的值加一，表示当前位置向上连续的'1'的个数加一。然后，计算以当前位置为右下角的矩形的面积，并更新 `area`。
    /// 6. 在计算矩形面积时，从当前列开始向左遍历，直到遇到高度为0的位置。在这个过程中，维护一个高度 `h` 和宽度 `w`。高度 `h` 是当前行的高度和之前行的高度的较小值，宽度 `w` 是当前位置到最左边的距离加一。每次迭代都更新面积 `area` 为当前面积和之前最大面积的较大值。
    /// 7. 遍历结束后，返回最大的矩形面积 `area`。
    ///
    /// 需要注意的是，这个函数在计算矩形面积时，只考虑了以每个'1'为右下角的矩形，而不是所有可能的矩形。这是因为对于每个'1'，它左边连续的'1'和上边连续的'1'构成了一个矩形，而这个矩形的最大高度就决定了以当前'1'为右下角的矩形的最大面积。同时，这个函数在计算过程中，通过动态维护高度和宽度，避免了重复计算，提高了效率。
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut heights = vec![vec![0; cols]; rows];
        let mut area = 0;
        for row in 0..rows {
            for col in 0..cols {
                if matrix[row][col] == '1' {
                    heights[row][col] = if row > 0 {
                        heights[row - 1][col] + 1
                    } else {
                        1
                    };

                    let mut h = rows;
                    let mut w = 1;
                    for i in (0..=col).rev() {
                        if heights[row][i] == 0 {
                            break;
                        }

                        h = h.min(heights[row][i]);
                        area = area.max(h * w);
                        w += 1;
                    }
                }
            }
        }
        area as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_maximal_rectangle() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        let out = Solution::maximal_rectangle(matrix);
        assert_eq!(out, 6);

        let matrix = vec![];
        let out = Solution::maximal_rectangle(matrix);
        assert_eq!(out, 0);

        let matrix = vec![vec!['0']];
        let out = Solution::maximal_rectangle(matrix);
        assert_eq!(out, 0);

        let matrix = vec![vec!['1']];
        let out = Solution::maximal_rectangle(matrix);
        assert_eq!(out, 1);

        let matrix = vec![vec!['0'], vec!['0']];
        let out = Solution::maximal_rectangle(matrix);
        assert_eq!(out, 0);
    }
}
