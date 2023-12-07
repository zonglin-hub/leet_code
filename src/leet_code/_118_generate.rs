//! 杨辉三角

use super::Solution;

impl Solution {
    /// f(i)(j) = f(i-1)(j-1) + f(i-1)(j)
    pub fn generate_118(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(num_rows as usize);
        for i in 0..num_rows as usize {
            res.push(vec![1; i + 1]);
            if i != 0 && i != 1 {
                for j in 1..i {
                    res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
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
    fn test_generate_118() {
        assert_eq!(Solution::generate_118(1), vec![vec![1]]);
        assert_eq!(Solution::generate_118(2), vec![vec![1], vec![1, 1]]);
    }
}
