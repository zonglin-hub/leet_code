//! 串联所有单词的子串

use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let s = s.as_bytes();
        let n = s.len();
        let k = words[0].len();
        if k > n {
            return vec![];
        }

        let mut out = vec![];
        let mut map =
            words
                .iter()
                .fold(HashMap::<&[u8], (usize, usize)>::new(), |mut map, word| {
                    map.entry(word.as_bytes()).or_default().0 += 1;
                    map
                });

        let mut map_is_reset = true;

        macro_rules! reset {
            () => {
                if !map_is_reset {
                    map_is_reset = true;
                    map.iter_mut().for_each(|(_, value)| value.1 = 0);
                }
            };
        }

        for i in 0..k {
            reset!();
            let (mut lo, mut hi) = (i, i);
            while hi <= n - k {
                match map.get_mut(&s[hi..hi + k]) {
                    None => {
                        hi += k;
                        lo = hi;
                        reset!();
                    }
                    Some(hi_value) => {
                        hi_value.1 += 1;
                        hi += k;
                        map_is_reset = false;

                        if hi_value.1 > hi_value.0 {
                            loop {
                                let lo_value = map.get_mut(&s[lo..lo + k]).unwrap();
                                lo += k;
                                lo_value.1 -= 1;

                                if lo_value.0 == lo_value.1 {
                                    break;
                                }
                            }
                        }
                    }
                }

                if hi - lo == words.len() * k {
                    out.push(lo as i32);
                    map.get_mut(&s[lo..lo + k]).unwrap().1 -= 1;
                    lo += k;
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_substring() {
        assert_eq!(
            vec![0, 9],
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            )
        );
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                [
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
                .to_vec()
            )
        );
    }
}
