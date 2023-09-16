//! 最长公共前缀
//!
//! 编写一个函数来查找字符串数组中的最长公共前缀。
//!
//! 如果不存在公共前缀，返回空字符串 ""。

use super::Solution;

impl Solution {
    pub fn longest_common_prefix_v1(strs: Vec<String>) -> String {
        strs.iter()
            .max()
            .expect("")
            .chars()
            .zip(strs.iter().min().expect("").chars())
            .take_while(|x| x.0 == x.1)
            .map(|x| x.0)
            .collect()
    }
}
