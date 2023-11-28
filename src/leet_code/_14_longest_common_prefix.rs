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
            .unwrap()
            .chars()
            .zip(strs.iter().min().unwrap().chars())
            .take_while(|x| x.0 == x.1)
            .map(|x| x.0)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_longest_common_prefix_v1() {
        assert_eq!(
            Solution::longest_common_prefix_v1(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
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
