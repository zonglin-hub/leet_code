//! ⚠️

use super::Solution;

struct Msg {
    rows: [u16; 9],
    cols: [u16; 9],
    blks: [[u16; 3]; 3],
    ok: bool,
}

impl Msg {
    fn new() -> Self {
        Self { rows: [0_u16; 9], cols: [0_u16; 9], blks: [[0_u16; 3]; 3], ok: false }
    }

    fn flip(&mut self, i: usize, j: usize, d: u8) {
        let d = d - 1;
        self.rows[i] ^= 1 << d;
        self.cols[j] ^= 1 << d;
        self.blks[i / 3][j / 3] ^= 1 << d;
    }

    fn valid_nums(&self, i: usize, j: usize) -> Vec<u8> {
        let mut ans = vec![];
        let mut b = !(self.rows[i] | self.cols[j] | self.blks[i / 3][j / 3]) & 0x1ff;
        while b > 0 {
            ans.push(b.trailing_zeros() as u8 + 1);
            b &= b - 1;
        }
        ans
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        #[inline]
        fn dfs(spaces: &[(usize, usize)], msg: &mut Msg, board: &mut Vec<Vec<char>>) {
            if spaces.is_empty() {
                msg.ok = true;
                return;
            }

            let (i, j) = spaces[0];

            for d in msg.valid_nums(i, j) {
                if msg.ok {
                    break;
                }

                board[i][j] = (d + 0x30) as char;
                msg.flip(i, j, d);

                dfs(&spaces[1..], msg, board);
                msg.flip(i, j, d);
            }
        }

        let mut msg = Msg::new();
        let mut spaces = vec![];

        board.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &c)| {
                if c == '.' {
                    spaces.push((i, j));
                } else {
                    let d = c as u8 - b'0';
                    msg.flip(i, j, d);
                }
            });
        });

        dfs(&spaces[..], &mut msg, board);
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_solve_sudoku() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);

        assert_eq!(
            board,
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
            ]
        );
    }
}
