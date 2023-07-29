#![allow(unused)]
pub struct Solution;

impl Solution {
    /// 杨辉三角
    /// f(i)(j) = f(i-1)(j-1) + f(i-1)(j)
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
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
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }
}
