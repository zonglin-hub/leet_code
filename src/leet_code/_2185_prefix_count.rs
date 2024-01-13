use super::Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|w| w.starts_with(&pref)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_prefix_count() {
        assert_eq!(
            Solution::prefix_count(
                vec![
                    "pay".to_owned(),
                    "attention".to_owned(),
                    "practice".to_owned(),
                    "attend".to_owned()
                ],
                "at".to_owned()
            ),
            2
        );
        assert_eq!(
            Solution::prefix_count(
                vec![
                    "leetcode".to_owned(),
                    "win".to_owned(),
                    "loops".to_owned(),
                    "success".to_owned()
                ],
                "code".to_owned()
            ),
            0
        );
    }
}
