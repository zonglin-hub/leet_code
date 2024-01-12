use super::Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn solve(
            board: &Vec<Vec<char>>,
            chars: &Vec<char>,
            visited: &mut Vec<Vec<bool>>,
            r: usize,
            c: usize,
            steps: usize,
        ) -> bool {
            let mut ret = false;
            if steps == chars.len() - 1 {
                return true;
            }

            for (r_, c_) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let r_next = r as i32 + r_;
                let c_next = c as i32 + c_;

                if r_next < 0
                    || r_next >= board.len() as i32
                    || c_next < 0
                    || c_next >= board[0].len() as i32
                {
                    continue;
                }

                let (r_next, c_next) = (r_next as usize, c_next as usize);

                if visited[r_next][c_next] {
                    continue;
                }

                if board[r_next][c_next] != chars[steps + 1] {
                    continue;
                }

                visited[r_next][c_next] = true;
                ret = solve(board, chars, visited, r_next, c_next, steps + 1);

                if ret {
                    break;
                }
                visited[r_next][c_next] = false;
            }

            ret
        }

        let cols = board[0].len();
        let rows = board.len();
        let chars = word.chars().collect::<Vec<char>>();
        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] != chars[0] {
                    continue;
                }
                let mut visited = vec![vec![false; cols]; rows];
                visited[r][c] = true;
                let ret = solve(&board, &chars, &mut visited, r, c, 0);
                if ret {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_subsets() {
        let board =
            vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']];
        assert!(Solution::exist(board.clone(), "ABCCED".to_string()));
        assert!(Solution::exist(board.clone(), "SEE".to_string()));
        assert!(Solution::exist(board.clone(), "SEE".to_string()));
    }
}
