use super::Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut res = vec![vec![0; n - 2]; n - 2];
        for i in 0..n - 2 {
            for j in 0..n - 2 {
                for x in i..i + 3 {
                    for y in j..j + 3 {
                        res[i][j] = std::cmp::max(res[i][j], grid[x][y]);
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ]),
            vec![vec![9, 9], vec![8, 6]]
        );
        assert_eq!(
            Solution::largest_local(vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 2, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]
        );
    }
}
