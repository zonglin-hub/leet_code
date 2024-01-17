use super::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        words
            .into_iter()
            .fold((HashSet::new(), 0), |(mut s, mut ans), w| {
                if s.contains(&w.chars().rev().collect::<String>()) {
                    ans += 1
                } else {
                    s.insert(w);
                }
                (s, ans)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_maximum_number_of_string_pairs() {
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec![
                "cd".to_owned(),
                "ac".to_owned(),
                "dc".to_owned(),
                "ca".to_owned(),
                "zz".to_owned()
            ]),
            2
        );
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec![
                "ab".to_owned(),
                "ba".to_owned(),
                "cc".to_owned()
            ]),
            1
        );
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec!["aa".to_owned(), "ab".to_owned()]),
            0
        );
    }
}
