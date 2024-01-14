//！作为子字符串出现在单词中的字符串数目

use super::Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.into_iter().filter(|s| word.contains(s)).count().try_into().unwrap()
    }

    pub fn num_of_strings_v1(patterns: Vec<String>, word: String) -> i32 {
        let mut nas = 0;
        for s in patterns.into_iter() {
            if word.contains(&s) {
                nas += 1;
            }
        }
        nas
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_num_of_strings() {
        assert_eq!(
            Solution::num_of_strings(
                vec!["a".to_string(), "abc".to_string(), "bc".to_string(), "d".to_string()],
                "abc".to_string()
            ),
            3
        );

        assert_eq!(
            Solution::num_of_strings(
                vec!["a".to_string(), "b".to_string(), "c".to_string(),],
                "aaaaabbbbb".to_string()
            ),
            2
        );

        assert_eq!(
            Solution::num_of_strings(
                vec!["a".to_string(), "a".to_string(), "a".to_string(),],
                "ab".to_string()
            ),
            3
        );
    }

    #[test]
    fn test_num_of_strings_v1() {
        assert_eq!(
            Solution::num_of_strings_v1(
                vec!["a".to_string(), "abc".to_string(), "bc".to_string(), "d".to_string()],
                "abc".to_string()
            ),
            3
        );

        assert_eq!(
            Solution::num_of_strings_v1(
                vec!["a".to_string(), "b".to_string(), "c".to_string(),],
                "aaaaabbbbb".to_string()
            ),
            2
        );

        assert_eq!(
            Solution::num_of_strings_v1(
                vec!["a".to_string(), "a".to_string(), "a".to_string(),],
                "ab".to_string()
            ),
            3
        );
    }
}
