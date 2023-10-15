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
        dfs(board, &rest, &mut row, &mut col, &mut block);
    }
}

fn dfs(board: &mut C, rest: &[(usize, usize)], row: &mut B, col: &mut B, block: &mut B) -> bool {
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
