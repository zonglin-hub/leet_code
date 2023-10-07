use super::Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = vec![vec![0; 10]; 9];
        let mut col = vec![vec![0; 10]; 9];
        let mut boxs = vec![vec![0; 10]; 9];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }

                let c = (board[i][j] as u8 - b'0') as usize;

                if row[i][c] != 0 || col[j][c] != 0 || boxs[(i / 3) * 3 + (j / 3)][c] != 0 {
                    return false;
                }

                row[i][c] += 1;
                col[j][c] += 1;
                boxs[(i / 3) * 3 + (j / 3)][c] += 1;
            }
        }
        true
    }
}
