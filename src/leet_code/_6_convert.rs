//! N 字形变换
//!
//! 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。

use std::convert::TryInto;

use super::Solution;

impl Solution {
    pub fn convert_v1(s: String, num_rows: i32) -> String {
        let mut rows = vec![String::new(); num_rows.try_into().expect("")];
        let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
        iter.zip(s.chars())
            .for_each(|(i, c)| rows[i as usize].push(c));
        rows.into_iter().collect()
    }
}
