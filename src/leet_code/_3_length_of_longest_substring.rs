//! 无重复字符的最长子串

use std::collections::HashSet;

impl super::Solution {
    /// 滑动窗口 (力扣官方题解)
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut max_len, mut current_len) = (0, 0);
        let mut charset = HashSet::new();
        let s = s.chars().collect::<Vec<char>>();
        let mut l = 0;
        s.iter().for_each(|c| {
            while charset.contains(c) {
                charset.remove(&s[l]);
                l += 1;
            }
            charset.insert(c);
            max_len = max_len.max(current_len - l + 1);
            current_len += 1;
        });
        max_len as i32
    }

    /// 滑动窗口 使用 HashMap
    pub fn length_of_longest_substring_v1(s: String) -> i32 {
        let (mut max_len, mut current_len) = (0, 0);
        let mut charset = std::collections::HashMap::new();
        let s = s.chars().collect::<Vec<char>>();
        let mut l = 0;
        s.iter().enumerate().for_each(|(i, c)| {
            match charset.get(c) {
                Some(&i) => {
                    for c in &s[l..=i] {
                        charset.remove(c);
                    }
                    current_len -= i - l;
                    l = i + 1;
                }
                None => {
                    current_len += 1;
                    max_len = max_len.max(current_len);
                }
            }
            charset.insert(*c, i);
        });
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }

    #[test]
    fn test_length_of_longest_substring_v1() {
        assert_eq!(Solution::length_of_longest_substring_v1("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring_v1("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring_v1("pwwkew".to_string()), 3);
    }
}
