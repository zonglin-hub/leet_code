//! 重新排列字符串
//!

use super::Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        if indices.len() != s.len() {
            return "".to_string();
        }

        let mut res = s.clone();
        let mut s = s.clone();

        unsafe {
            for (idx, _) in indices.iter().enumerate() {
                res.as_mut_vec()[indices[idx] as usize] = s.as_mut_vec()[idx];
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_restore_string() {
        assert_eq!(
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        assert_eq!(
            Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc".to_string()
        );
    }
}
