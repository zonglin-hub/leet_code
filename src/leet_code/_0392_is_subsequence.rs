//! 判断子序列
//!

use super::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut counter_s = 0;
        let mut counter_t: usize = 0;
        for i in s.chars() {
            for c in t[counter_t..].chars() {
                counter_t += 1;
                if c == i {
                    counter_s += 1;
                    if counter_s == s.len() {
                        return true;
                    }
                    break;
                }
            }
        }
        false
    }

    pub fn is_subsequence_v1(s: String, t: String) -> bool {
        let (mut i, mut j) = (0, 0);
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        while i < s.len() && j < t.len() {
            if s_bytes[i] == t_bytes[j] {
                i += 1;
            }
            j += 1;
        }
        i == s.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_subsequence() {
        assert!(Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()));
        assert!(!Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()));
        assert!(!Solution::is_subsequence("acb".to_string(), "ahbgdc".to_string()));
    }

    #[test]
    fn test_is_subsequence_v1() {
        assert!(Solution::is_subsequence_v1("abc".to_string(), "ahbgdc".to_string()));
        assert!(!Solution::is_subsequence_v1("axc".to_string(), "ahbgdc".to_string()));
        assert!(!Solution::is_subsequence_v1("acb".to_string(), "ahbgdc".to_string()));
    }
}
