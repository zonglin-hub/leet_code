//! N 字形变换
//!
//! 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。

use std::convert::TryInto;

use crate::types::base_type::Solution;

impl Solution {
    pub fn convert_v1(s: String, num_rows: i32) -> String {
        let mut rows = vec![String::new(); num_rows.try_into().expect("")];
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
    fn test_convert_v1() {
        /*
            输入：s = "PAYPALISHIRING", numRows = 3
            输出："PAHNAPLSIIGYIR"
        */
        assert_eq!(
            Solution::convert_v1(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );

        /*
            输入：s = "PAYPALISHIRING", numRows = 4
            输出："PINALSIGYAHRPI"
            解释：
            P     I    N
            A   L S  I G
            Y A   H R
            P     I
        */
        assert_eq!(
            Solution::convert_v1(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );

        /*
            输入：s = "A", numRows = 1
            输出："A"
        */
        assert_eq!(
            Solution::convert_v1(String::from("A"), 1),
            String::from("A")
        );
    }
}
