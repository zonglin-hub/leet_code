//! 可以攻击国王的皇后

use super::Solution;

impl Solution {
    pub fn queens_attackthe_king_v1(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut board = vec![vec![0; 8]; 8];
        for queen in queens {
            board[queen[0] as usize][queen[1] as usize] = 1;
        }

        let mut result = vec![];
        let dx = [-1, -1, 0, 1, 1, 1, 0, -1];
        let dy = [0, 1, 1, 1, 0, -1, -1, -1];
        for i in 0..8 {
            let mut x = king[0];
            let mut y = king[1];
            loop {
                x += dx[i];
                y += dy[i];
                if x < 0 || y < 0 || x >= 8 || y >= 8 {
                    break;
                }
                if board[x as usize][y as usize] == 1 {
                    result.push(vec![x, y]);
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{expected_sort, expected_sort_vec, Solution};

    #[test]
    fn test_queens_attackthe_king_v1() {
        assert_eq!(
            expected_sort(Solution::queens_attackthe_king_v1(
                expected_sort_vec(vec![[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]]),
                vec![0, 0],
            )),
            expected_sort(expected_sort_vec(vec![[0, 1], [1, 0], [3, 3]]))
        );
        assert_eq!(
            expected_sort(Solution::queens_attackthe_king_v1(
                expected_sort_vec(vec![[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]],),
                vec![3, 3],
            )),
            expected_sort(expected_sort_vec(vec![[2, 2], [3, 4], [4, 4]]))
        );
        assert_eq!(
            expected_sort(Solution::queens_attackthe_king_v1(
                expected_sort_vec(vec![
                    [5, 6],
                    [7, 7],
                    [2, 1],
                    [0, 7],
                    [1, 6],
                    [5, 1],
                    [3, 7],
                    [0, 3],
                    [4, 0],
                    [1, 2],
                    [6, 3],
                    [5, 0],
                    [0, 4],
                    [2, 2],
                    [1, 1],
                    [6, 4],
                    [5, 4],
                    [0, 0],
                    [2, 6],
                    [4, 5],
                    [5, 2],
                    [1, 4],
                    [7, 5],
                    [2, 3],
                    [0, 5],
                    [4, 2],
                    [1, 0],
                    [2, 7],
                    [0, 1],
                    [4, 6],
                    [6, 1],
                    [0, 6],
                    [4, 3],
                    [1, 7]
                ],),
                vec![3, 4],
            )),
            expected_sort(expected_sort_vec(vec![
                [2, 3],
                [1, 4],
                [1, 6],
                [3, 7],
                [4, 3],
                [5, 4],
                [4, 5]
            ]))
        );
    }
}
