//! 无重复字符的最长子串

use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring_v1(s: String) -> i32 {
        let (mut ans, mut cnt) = (0, 0);
        let mut map = HashMap::new();
        let s = s.chars().collect::<Vec<_>>();
        let mut l = 0;

        s.iter().enumerate().for_each(|(i, c)| {
            match map.get(c) {
                None => {
                    cnt += 1;
                    ans = ans.max(cnt);
                }
                Some(&i) => {
                    for c in &s[l..=i] {
                        map.remove(c);
                    }
                    cnt -= i - l;
                    l = i + 1;
                }
            }
            map.insert(*c, i);
        });
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_length_of_longest_substring_v1() {
        assert_eq!(
            Solution::length_of_longest_substring_v1("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_v1("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring_v1("pwwkew".to_string()),
            3
        );
    }
}
