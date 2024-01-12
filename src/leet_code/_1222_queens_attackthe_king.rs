//! 可以攻击国王的皇后

use super::Solution;

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
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
    use crate::leet_code::Solution;

    #[test]
    fn test_queens_attackthe_king_v1() {
        assert_eq!(
            Solution::queens_attackthe_king(
                vec![vec![0, 1], vec![1, 0], vec![4, 0], vec![0, 4], vec![3, 3], vec![2, 4]],
                vec![0, 0],
            ),
            vec![vec![0, 1], vec![3, 3], vec![1, 0]]
        );
        assert_eq!(
            Solution::queens_attackthe_king(
                vec![
                    vec![0, 0],
                    vec![1, 1],
                    vec![2, 2],
                    vec![3, 4],
                    vec![3, 5],
                    vec![4, 4],
                    vec![4, 5]
                ],
                vec![3, 3],
            ),
            vec![vec![3, 4], vec![4, 4], vec![2, 2]]
        );
        assert_eq!(
            Solution::queens_attackthe_king(
                vec![
                    vec![5, 6],
                    vec![7, 7],
                    vec![2, 1],
                    vec![0, 7],
                    vec![1, 6],
                    vec![5, 1],
                    vec![3, 7],
                    vec![0, 3],
                    vec![4, 0],
                    vec![1, 2],
                    vec![6, 3],
                    vec![5, 0],
                    vec![0, 4],
                    vec![2, 2],
                    vec![1, 1],
                    vec![6, 4],
                    vec![5, 4],
                    vec![0, 0],
                    vec![2, 6],
                    vec![4, 5],
                    vec![5, 2],
                    vec![1, 4],
                    vec![7, 5],
                    vec![2, 3],
                    vec![0, 5],
                    vec![4, 2],
                    vec![1, 0],
                    vec![2, 7],
                    vec![0, 1],
                    vec![4, 6],
                    vec![6, 1],
                    vec![0, 6],
                    vec![4, 3],
                    vec![1, 7]
                ],
                vec![3, 4],
            ),
            vec![
                vec![1, 4],
                vec![1, 6],
                vec![3, 7],
                vec![4, 5],
                vec![5, 4],
                vec![4, 3],
                vec![2, 3]
            ]
        );
    }
}
