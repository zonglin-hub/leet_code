use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut freq1 = HashMap::new();
        let mut freq2 = HashMap::new();

        words1
            .into_iter()
            .for_each(|x| *freq1.entry(x).or_insert(0) += 1);

        words2
            .into_iter()
            .for_each(|x| *freq2.entry(x).or_insert(0) += 1);

        let mut res = 0;

        for (k, v) in freq1 {
            if let Some(&i) = freq2.get(&k) {
                if v == 1 && i == 1 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_count_words() {
        assert_eq!(
            Solution::count_words(
                vec![
                    "leetcode".to_owned(),
                    "is".to_owned(),
                    "amazing".to_owned(),
                    "as".to_owned(),
                    "is".to_owned()
                ],
                vec!["amazing".to_owned(), "leetcode".to_owned(), "is".to_owned()]
            ),
            2
        );
        assert_eq!(
            Solution::count_words(
                vec!["b".to_owned(), "bb".to_owned(), "bbb".to_owned()],
                vec!["a".to_owned(), "aa".to_owned(), "aaa".to_owned()]
            ),
            0
        );
        assert_eq!(
            Solution::count_words(
                vec!["a".to_owned(), "ab".to_owned()],
                vec![
                    "a".to_owned(),
                    "a".to_owned(),
                    "a".to_owned(),
                    "ab".to_owned()
                ]
            ),
            1
        );
    }
}
