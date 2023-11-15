use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map = HashMap::new();

        for i in t.as_bytes() {
            *map.entry(i).or_insert(0) += 1;
        }

        let ss = s.as_bytes();
        let mut i = 0;
        let n = ss.len();
        let mut cnt = t.len();
        let mut start = 0;
        let mut end = 0;
        let mut range = 0; // New variable to hold the range

        for j in 0..n {
            if let Some(p) = map.get_mut(&ss[j]) {
                *p -= 1;
                if *p >= 0 {
                    cnt -= 1;
                }
            }

            while cnt == 0 {
                if range < n {
                    range = j - i;
                    start = i;
                    end = j + 1;
                }

                if let Some(p) = map.get_mut(&ss[i]) {
                    *p += 1;
                    if *p > 0 {
                        cnt += 1;
                    }
                }
                i += 1;
            }
        }
        s[start..end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_window() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let expected = Solution::min_window(s, t);
        let out = "BANC".to_string();
        assert_eq!(expected, out);

        let s = "a".to_string();
        let t = "a".to_string();
        let expected = Solution::min_window(s, t);
        let out = "a".to_string();
        assert_eq!(expected, out);

        let s = "a".to_string();
        let t = "aa".to_string();
        let expected = Solution::min_window(s, t);
        let out = "".to_string();
        assert_eq!(expected, out);
    }

    #[test]
    fn test_entry() {
        use std::collections::HashMap;

        let mut letters = HashMap::new();

        for ch in "a short treatise on fungi".chars() {
            letters
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        assert_eq!(letters[&'s'], 2);
        assert_eq!(letters[&'t'], 3);
        assert_eq!(letters[&'u'], 1);
        assert_eq!(letters.get(&'y'), None);
    }

    #[test]
    fn test_or_insert() {
        use std::collections::HashMap;

        let mut map: HashMap<&str, u32> = HashMap::new();
        // 个表达式会查找键为"poneyland"的条目。如果找到这个条目，它就返回这个条目的引用，否则它会插入值10并返回这个新插入条目的引用。
        // 然后，*= 2 操作将找到（或插入的）值乘以2。
        // 所以，如果"poneyland"这个键之前不存在于map中，那么这行代码会插入值10，然后乘以2，结果为20。如果"poneyland"这个键已经存在，那么这行代码会将其值乘以2。
        map.entry("poneyland").or_insert(3);
        assert_eq!(map["poneyland"], 3);

        *map.entry("poneyland").or_insert(10) *= 2;
        assert_eq!(map["poneyland"], 6);
    }
}
