//! 有效的数独

use super::Solution;

impl Solution {
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
        assert_eq!("hello".chars().next().unwrap(), 'h');
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
        assert!(!Solution::is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
    }
}
