use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        words
            .iter()
            .filter(|w| {
                let mut res = HashMap::new();
                let mut res2 = res.clone();
                w.chars().zip(pattern.chars()).all(|(i, j)| {
                    *res.entry(i).or_insert(j) == j && *res2.entry(j).or_insert(i) == i
                })
            })
            .map(|f| f.to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::find_and_replace_pattern(
                vec![
                    "abc".to_owned(),
                    "deq".to_owned(),
                    "mee".to_owned(),
                    "aqq".to_owned(),
                    "dkd".to_owned(),
                    "ccc".to_owned()
                ],
                "".to_owned()
            ),
            vec![
                "abc".to_owned(),
                "deq".to_owned(),
                "mee".to_owned(),
                "aqq".to_owned(),
                "dkd".to_owned(),
                "ccc".to_owned()
            ]
        );
    }
}
