#![allow(unused)]

use std::convert::TryInto;

struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/zigzag-conversion/
    ///
    /// N 字形变换
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut rows = vec![String::new(); num_rows.try_into().unwrap()];
        let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
        iter.zip(s.chars())
            .for_each(|(i, c)| rows[i as usize].push(c));
        rows.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
        assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"));
    }
}
