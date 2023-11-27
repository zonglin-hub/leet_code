use super::Solution;

impl Solution {
    /// 这个函数是用于解决N皇后问题的算法。N皇后问题是一个经典的回溯问题，目标是在N*N的棋盘上放置N个皇后，使得它们彼此不能相互攻击。即任何两个皇后都不能处于同一行、同一列或同一斜线上。
    ///
    /// 总的来说，这个函数使用回溯法寻找N皇后问题的所有解决方案，并以字符串的形式返回它们，其中每一行代表棋盘的一行，每个'.'代表一个空格，每个'Q'代表一个皇后。
    pub fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
        /// `columns` 是一个整数向量，表示每行皇后的列位置。例如，如果`columns`是[0, 2, 1, 3]，则表示第一个皇后在第一行的第一列，第二个皇后在第二行的第三列，依此类推。
        /// 该函数通过`columns`生成一个字符串向量，其中每个字符串代表一行皇后。如果当前位置有皇后（即列位置是有效的），则用'Q'表示，否则用'.'表示。
        fn format(columns: &Vec<i32>) -> Vec<String> {
            let n = columns.len();
            let mut all_rows = Vec::new();
            for (i, _) in columns.iter().enumerate().take(n) {
                let mut each_row = String::new();
                for j in 0..n as i32 {
                    each_row.push(if columns[i] == j { 'Q' } else { '.' });
                }
                all_rows.push(each_row);
            }
            all_rows
        }

        /// 检查在给定的行和列上是否可以放置一个皇后，而不与其他已经放置的皇后冲突。
        /// 通过检查同一列或对角线上的其他皇后来决定。如果在同一列或对角线上有任何其他皇后，函数返回false。
        fn _check(row: usize, col: i32, columns: &[i32]) -> bool {
            for (r, _) in columns.iter().enumerate().take(row) {
                if columns[r] == col || row - r == (columns[r] - col).unsigned_abs() as usize {
                    return false;
                }
            }
            true
        }
        fn check(row: usize, col: i32, columns: &[i32]) -> bool {
            columns
                .iter()
                .enumerate()
                .take(row)
                .filter(|&(r, _)| {
                    columns[r] == col || row - r == (columns[r] - col).unsigned_abs() as usize
                })
                .all(|(c, _)| c == row)
        }

        /// 这是一个递归函数，用于回溯并找到所有可能的解决方案。
        /// 在每一行中，它尝试在每一列上放置一个皇后，并递归地调用自身来放置下一个皇后。
        /// 如果在某一行不能放置皇后（即`check`函数返回false），则撤销该行的皇后并尝试下一列。
        /// 当所有的皇后都被成功放置时（即到达了最后一行），它把这一解决方案添加到结果集中。
        fn backtracking(n: usize, row: usize, columns: &mut Vec<i32>, res: &mut Vec<Vec<String>>) {
            if row == n {
                return res.push(format(columns));
            }
            for col in 0..n as i32 {
                columns[row] = col;
                if check(row, col, columns) {
                    backtracking(n, row + 1, columns, res);
                }
                columns[row] = -1;
            }
        }

        let mut res = Vec::new();
        let mut columns = vec![-1; n];
        backtracking(n, 0, &mut columns, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_my_pow() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q"]]);
    }

    #[test]
    fn test_vec() {
        let a = [1, 3, 4, 2];
        let n = a.len();
        for (i, _) in a.iter().enumerate().take(n) {
            println!("ele: {}", i);
        }

        for i in a.iter().take(n) {
            println!("ele: {}", i);
        }

        for i in 0..n {
            println!("{}", i);
        }
    }
}
