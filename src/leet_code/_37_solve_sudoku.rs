//! ⚠️

use super::Solution;

type B = Vec<Vec<bool>>;
type C = Vec<Vec<char>>;

impl Solution {
    pub fn solve_sudoku(board: &mut C) {
        let mut row = vec![vec![false; 9]; 9];
        let mut col = vec![vec![false; 9]; 9];
        let mut block = vec![vec![false; 9]; 9];
        let mut rest = vec![];
        for i in 0..9 {
            for j in 0..9 {
                match board[i][j] {
                    '.' => rest.push((i, j)),
                    _ => {
                        let n = (board[i][j] as u8 - b'1') as usize;
                        row[i][n] = true;
                        col[j][n] = true;
                        block[i / 3 * 3 + j / 3][n] = true;
                    }
                }
            }
        }

        fn dfs(
            board: &mut C,
            rest: &[(usize, usize)],
            row: &mut B,
            col: &mut B,
            block: &mut B,
        ) -> bool {
            if let Some((i, j)) = rest.first() {
                let (i, j) = (*i, *j);
                for x in 0..9 {
                    if !row[i][x] && !col[j][x] && !block[i / 3 * 3 + j / 3][x] {
                        row[i][x] = true;
                        col[j][x] = true;
                        block[i / 3 * 3 + j / 3][x] = true;
                        board[i][j] = (x as u8 + b'1') as char;
                        if dfs(board, &rest[1..], row, col, block) {
                            return true;
                        }
                        row[i][x] = false;
                        col[j][x] = false;
                        block[i / 3 * 3 + j / 3][x] = false;
                    }
                }
                false
            } else {
                true
            }
        }

        dfs(board, &rest, &mut row, &mut col, &mut block);
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{expected_sort_vec_char, Solution};

    #[test]
    fn test_solve_sudoku() {
        let mut b = expected_sort_vec_char(vec![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);

        // println!("{:?}", b);
        Solution::solve_sudoku(&mut b);
        // println!("{:?}", b);

        assert!(
            b == expected_sort_vec_char(vec![
                ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                ['3', '4', '5', '2', '8', '6', '1', '7', '9'],
            ])
        );
    }
}
