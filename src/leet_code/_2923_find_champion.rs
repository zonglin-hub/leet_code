use super::Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        for i in 0..n {
            let sum = grid[i].iter().sum::<i32>();

            if sum == n as i32 - 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::find_champion(vec![vec![0, 1], vec![0, 0]]), 0);
        assert_eq!(Solution::find_champion(vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]]), 1);
    }
}
