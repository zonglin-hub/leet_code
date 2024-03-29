use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let a = "qwertyuiop".chars().collect::<HashSet<_>>();
        let b = "asdfghjkl".chars().collect::<HashSet<_>>();
        let c = "zxcvbnm".chars().collect::<HashSet<_>>();
        let mut ans = vec![];
        for word in words {
            let t = word.to_lowercase().chars().collect::<HashSet<_>>();
            if t.is_subset(&a) || t.is_subset(&b) || t.is_subset(&c) {
                ans.push(word);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_words() {
        assert_eq!(
            Solution::find_words(vec![
                "Hello".to_owned(),
                "Alaska".to_owned(),
                "Dad".to_owned(),
                "Peace".to_owned()
            ]),
            vec!["Alaska".to_owned(), "Dad".to_owned()]
        );
    }
}
