//! 可以攻击国王的皇后

use super::Solution;

impl Solution {
    pub fn queens_attackthe_king_v1(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        // 创建一个8*8的二维数组，用于存放棋盘
        let mut board = vec![vec![0; 8]; 8];
        // 遍历棋盘中的每一个棋子
        for queen in queens {
            // 将棋子放置在棋盘中的位置上
            board[queen[0] as usize][queen[1] as usize] = 1;
        }

        // 创建一个空的结果数组
        let mut result = vec![];
        // 定义横向和纵向的偏移量
        let dx = [-1, -1, 0, 1, 1, 1, 0, -1];
        let dy = [0, 1, 1, 1, 0, -1, -1, -1];
        // 遍历棋盘中的每一个位置
        for i in 0..8 {
            // 定义棋盘的当前位置
            let mut x = king[0];
            let mut y = king[1];
            // 循环遍历棋盘中的每一个位置
            loop {
                // 将棋盘的当前位置放置在棋盘中的位置上
                x += dx[i];
                y += dy[i];
                // 如果当前位置超出范围，则跳出循环
                if x < 0 || y < 0 || x >= 8 || y >= 8 {
                    break;
                }
                // 如果当前位置已经被占用，则跳出循环
                if board[x as usize][y as usize] == 1 {
                    result.push(vec![x, y]);
                    break;
                }
            }
        }
        result
    }
}
