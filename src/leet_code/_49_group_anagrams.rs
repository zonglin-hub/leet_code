//! 字母异位词分组

use super::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_map = HashMap::new();

        strs.into_iter().for_each(|str| {
            let mut ch = str.chars().collect::<Vec<char>>();
            ch.sort_unstable();

            let vs = vec![];
            let os = ch.iter().collect::<String>();
            let val = hash_map.entry(os).or_insert(vs);
            val.push(str);
        });

        hash_map.values().fold(vec![], |mut res, v| {
            res.push(v.to_owned());
            res
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_group_anagrams() {
        assert_eq!(
            Solution::group_anagrams(vec!["".to_string(),]),
            vec![vec![""]]
        );
        assert_eq!(
            Solution::group_anagrams(vec!["a".to_string(),]),
            vec![vec!["a"]]
        );
    }
}
