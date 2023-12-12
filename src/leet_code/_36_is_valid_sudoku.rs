//! 有效的数独

use super::Solution;

impl Solution {
    // pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    //     let mut line = vec![0; 9];
    //     let mut row = vec![0; 9];
    //     let mut square = vec![vec![0; 3]; 3];
    //     for i in 0..9 {
    //         for j in 0..9 {
    //             if board[i][j] != '.' {
    //                 let bit = 1 << board[i][j] as u8 - b'1';

    //                 if line[i] & bit != 0 || row[j] & bit != 0 || square[i / 3][j / 3] & bit != 0 {
    //                     return false;
    //                 }

    //                 line[i] |= bit;
    //                 row[j] |= bit;
    //                 square[i / 3][j / 3] |= bit;
    //             }
    //         }
    //     }
    //     true
    // }

    // pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    //     let mut rows = vec![0; 9];
    //     let mut cols = vec![0; 9];
    //     let mut blks = vec![vec![0; 3]; 3];
    //     for i in 0..9 {
    //         for j in 0..9 {
    //             if board[i][j] == '.' {
    //                 continue;
    //             }

    //             let n = board[i][j] as u8 - b'0';

    //             if rows[i] & (1 << n) != 0
    //                 || cols[j] & (1 << n) != 0
    //                 || blks[i / 3][j / 3] & (1 << n) != 0
    //             {
    //                 return false;
    //             }

    //             rows[i] ^= 1 << n;
    //             cols[j] ^= 1 << n;
    //             blks[i / 3][j / 3] ^= 1 << n;
    //         }
    //     }
    //     true
    // }

    // pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    //     let mut row = vec![vec![0; 10]; 9];
    //     let mut col = vec![vec![0; 10]; 9];
    //     let mut boxs = vec![vec![0; 10]; 9];
    //     for i in 0..9 {
    //         for j in 0..9 {
    //             if board[i][j] == '.' {
    //                 continue;
    //             }

    //             let c = (board[i][j] as u8 - b'0') as usize;
    //             if row[i][c] != 0 || col[j][c] != 0 || boxs[(i / 3) * 3 + (j / 3)][c] != 0 {
    //                 return false;
    //             }

    //             row[i][c] += 1;
    //             col[j][c] += 1;
    //             boxs[(i / 3) * 3 + (j / 3)][c] += 1;
    //         }
    //     }
    //     true
    // }

    #[allow(clippy::needless_range_loop)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![vec![0; 10]; 9];
        let mut columns = vec![vec![0; 10]; 9];
        let mut subboxes = vec![vec![vec![0; 10]; 9]; 9];

        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j] as u8;

                if c != b'.' {
                    let index = (c - b'0' - 1) as usize;
                    rows[i][index] += 1;
                    columns[j][index] += 1;
                    subboxes[i / 3][j / 3][index] += 1;

                    if rows[i][index] > 1
                        || columns[j][index] > 1
                        || subboxes[i / 3][j / 3][index] > 1
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_valid_sudoku() {
        assert_eq!("hello".chars().nth(0).unwrap(), 'h');
        assert!(Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
    }
}
