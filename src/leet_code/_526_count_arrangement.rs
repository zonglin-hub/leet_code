//! 优美的排列

use super::Solution;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        fn dfs_count_arrangement(i: i32, n: i32, used: &mut Vec<bool>) -> i32 {
            if i > n {
                return 1;
            }

            let mut sum = 0;

            for j in 1..=n {
                if !used[j as usize] && (j % i == 0 || i % j == 0) {
                    used[j as usize] = true;
                    sum += dfs_count_arrangement(i + 1, n, used);
                    used[j as usize] = false;
                }
            }
            sum
        }

        dfs_count_arrangement(1, n, &mut vec![false; n as usize + 1])
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_count_arrangement() {
        assert_eq!(Solution::count_arrangement(2), 2);
        assert_eq!(Solution::count_arrangement(1), 1);
    }
}
