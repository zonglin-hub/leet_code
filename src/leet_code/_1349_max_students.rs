use super::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        fn is_single_row_compliant(
            seats: &[Vec<char>],
            status: usize,
            n: usize,
            row: usize,
        ) -> bool {
            // for j in 0..n {
            //     if (status >> j) & 1 == 1 {
            //         if seats[row][j] == '#' {
            //             return false;
            //         }
            //         if j > 0 && (status >> (j - 1)) & 1 == 1 {
            //             return false;
            //         }
            //     }
            // }
            // true 等同实现逻辑
            (0..n).all(|j| {
                if (status & (1 << j)) == 0 {
                    true
                } else {
                    seats[row][j] != '#' && (j == 0 || (status & (1 << (j - 1))) == 0)
                }
            })
        }

        fn is_cross_rows_compliant(status: usize, upper_row_status: usize, n: usize) -> bool {
            // for j in 0..n {
            //     if (status >> j) & 1 == 1 {
            //         if j > 0 && (upper_row_status >> (j - 1)) & 1 == 1 {
            //             return false;
            //         }
            //         if j < n - 1 && (upper_row_status >> (j + 1)) & 1 == 1 {
            //             return false;
            //         }
            //     }
            // }
            // true 等同实现逻辑
            (0..n).all(|j| {
                if (status & (1 << j)) == 0 {
                    true
                } else {
                    // 检查当前行的第 j 个座位是否被占用
                    // 如果被占用，检查上一行的第 j-1 个座位是否未被占用（对于 j > 0 的情况）
                    // 或者检查下一行的第 j+1 个座位是否未被占用（对于 j < n - 1 的情况）
                    (j == 0 || (upper_row_status & (1 << (j - 1))) == 0)
                        && (j == n - 1 || (upper_row_status & (1 << (j + 1))) == 0)
                }
            })
        }

        fn bit_count(mut bits: usize) -> i32 {
            bits = bits - ((bits >> 1) & 0x55555555);
            bits = (bits & 0x33333333) + ((bits >> 2) & 0x33333333);
            bits = (bits + (bits >> 4)) & 0x0f0f0f0f;
            bits = (bits + (bits >> 8)) & 0x00ff00ff;
            bits = (bits + (bits >> 16)) & 0x0000ffff;
            bits as i32
        }

        fn dp(
            seats: &Vec<Vec<char>>,
            row: usize,
            status: usize,
            n: usize,
            memo: &mut HashMap<i64, i32>,
        ) -> i32 {
            // let key = ((row << n) + status) as i64; 等同实现逻辑
            let key = ((row << n) | status) as i64;

            if !memo.contains_key(&key) {
                if !is_single_row_compliant(seats, status, n, row) {
                    memo.insert(key, i32::MIN);
                    return i32::MIN;
                }

                let students = bit_count(status);

                if row == 0 {
                    memo.insert(key, students);
                    return students;
                }

                let mut mx = 0;
                for upper_row_status in 0..(1 << n) {
                    if is_cross_rows_compliant(status, upper_row_status, n) {
                        mx = mx.max(dp(seats, row - 1, upper_row_status, n, memo));
                    }
                }
                memo.insert(key, students + mx);
            }
            *memo.get(&key).unwrap()
        }

        let mut mx = 0;
        let m = seats.len();
        let n = seats[0].len();
        let mut memo = HashMap::<i64, i32>::new();
        (0..(1 << n)).for_each(|i| mx = mx.max(dp(&seats, m - 1, i, n, &mut memo)));
        mx
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_max_students() {
        assert_eq!(
            Solution::max_students(vec![
                vec!['#', '.', '#', '#', '.', '#'],
                vec!['.', '#', '#', '#', '#', '.'],
                vec!['#', '.', '#', '#', '.', '#']
            ]),
            4
        );
        assert_eq!(
            Solution::max_students(vec![
                vec!['.', '#'],
                vec!['#', '#'],
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['.', '#']
            ]),
            3
        );
        assert_eq!(
            Solution::max_students(vec![
                vec!['#', '.', '.', '.', '#'],
                vec!['.', '#', '.', '#', '.'],
                vec!['.', '.', '#', '.', '.'],
                vec!['.', '#', '.', '#', '.'],
                vec!['#', '.', '.', '.', '#']
            ]),
            10
        );
    }
}
