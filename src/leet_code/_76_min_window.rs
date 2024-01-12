use super::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map = HashMap::new();
        for c in t.chars() {
            let cnt = map.entry(c).or_insert(0);
            *cnt += 1;
        }

        let mut ans = "";
        let ch = s.chars().collect::<Vec<_>>();
        let mut ans_len = s.len();
        let mut count = t.len();
        let mut l = 0;
        for r in 0..ch.len() {
            if !map.contains_key(&ch[r]) {
                continue;
            }

            if let Some(n) = map.get_mut(&ch[r]) {
                *n -= 1;
                if *n >= 0 {
                    count -= 1;
                }
            }

            while count == 0 {
                if ans_len > r - l {
                    ans = &s[l..=r];
                    ans_len = r - l + 1;
                }

                if !map.contains_key(&ch[l]) {
                    l += 1;
                    continue;
                }

                if let Some(n) = map.get_mut(&ch[l]) {
                    *n += 1;
                    if *n > 0 {
                        count += 1;
                    }
                }

                l += 1;
            }
        }

        ans.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_window() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
        assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a".to_string());
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "".to_string());
    }
}
