use super::Solution;

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        if words.len() != s.len() {
            return false;
        }

        for (i, v) in words.iter().enumerate() {
            if v.chars().next() != s.chars().nth(i) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_acronym() {
        assert!(Solution::is_acronym(
            vec!["alice".to_string(), "bob".to_string(), "charlie".to_string()],
            "abc".to_string()
        ));
        assert!(Solution::is_acronym(
            vec![
                "never".to_string(),
                "gonna".to_string(),
                "give".to_string(),
                "up".to_string(),
                "on".to_string(),
                "you".to_string()
            ],
            "ngguoy".to_string()
        ));
        assert!(!Solution::is_acronym(
            vec!["an".to_string(), "apple".to_string()],
            "a".to_string()
        ));
    }
}
