//! 杨辉三角 ||

use crate::types::base_type::Solution;

impl Solution {
    /// 定义一个长度为 rowIndex + 1 的一维数组 res
    /// 从后向前计算，每次更新 `res[j]` 即可。
    /// 具体而言，第 j 列的值更新为第 j-1 列的值加上第 j 列的值，即 `res[j] = res[j - 1] + res[j]`
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![1; (row_index + 1) as usize];
        for i in 1..=row_index as usize {
            for j in (1..i).rev() {
                res[j] += res[j - 1]
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
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }
}
