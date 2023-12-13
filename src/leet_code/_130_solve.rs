use super::Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
            if board[r][c] != 'O' {
                return;
            }

            board[r][c] = '1';

            if r - 1 < board.len() && board[r - 1][c] == 'O' {
                dfs(board, r - 1, c);
            }

            if r + 1 < board.len() && board[r + 1][c] == 'O' {
                dfs(board, r + 1, c);
            }

            if c - 1 < board[0].len() && board[r][c - 1] == 'O' {
                dfs(board, r, c - 1);
            }

            if c + 1 < board[0].len() && board[r][c + 1] == 'O' {
                dfs(board, r, c + 1);
            }
        }

        (0..board[0].len()).for_each(|i| {
            dfs(board, 0, i);
            dfs(board, board.len() - 1, i)
        });

        (0..board.len()).for_each(|i| {
            dfs(board, i, 0);
            dfs(board, i, board[0].len() - 1);
        });

        for row in board {
            for i in 0..row.len() {
                match row[i] {
                    'O' => row[i] = 'X',
                    '1' => row[i] = 'O',
                    _ => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_solve() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);

        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X']
            ]
        );

        let mut board = vec![vec!['X']];
        Solution::solve(&mut board);
        assert_eq!(board, vec![vec!['X']]);
    }
}
