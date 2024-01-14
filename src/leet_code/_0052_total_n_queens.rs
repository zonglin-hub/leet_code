use super::Solution;

impl Solution {
    pub fn total_n_queens(n: usize) -> i32 {
        fn format(columns: &[i32]) -> Vec<String> {
            let n = columns.len();
            let all_rows = Vec::new();

            let each_row = |mut all: Vec<String>, i: usize| {
                let mut each_row = String::new();
                for j in 0..n as i32 {
                    each_row.push(if columns[i] == j { 'Q' } else { '.' });
                }
                all.push(each_row);
            };

            for (i, _) in columns.iter().enumerate().take(n) {
                each_row(all_rows.clone(), i);
            }

            all_rows
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
        res.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_my_pow() {
        assert_eq!(Solution::total_n_queens(1), 1);
        assert_eq!(Solution::total_n_queens(4), 2);
    }
}
