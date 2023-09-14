//! 最长公共前缀
//!
//! 编写一个函数来查找字符串数组中的最长公共前缀。
//!
//! 如果不存在公共前缀，返回空字符串 ""。

use crate::types::base_type::Solution;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix_v1() {
        /*
            输入：strs = ["flower","flow","flight"]
            输出："fl"
        */
        assert_eq!(
            Solution::longest_common_prefix_v1(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );

        /*
            输入：strs = ["dog","racecar","car"]
            输出：""
            解释：输入不存在公共前缀。
        */
        assert_eq!(
            Solution::longest_common_prefix_v1(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }
}
