use super::Solution;

impl Solution {
    /// 模拟(力扣官方题解)
    pub fn split_words_by_separator_fn(words: Vec<String>, separator: char) -> Vec<String> {
        words
            .iter()
            .flat_map(|s| s.split(separator).collect::<Vec<_>>())
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn split_words_by_separator_command(words: Vec<String>, separator: char) -> Vec<String> {
        let mut ans = vec![];
        for word in words {
            for w in word.split(separator) {
                if !w.is_empty() {
                    ans.push(w.to_string());
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_split_words_by_separator_fn() {
        assert_eq!(
            Solution::split_words_by_separator_fn(
                vec!["one.two.three".to_owned(), "four.five".to_owned(), "six".to_owned()],
                '.'
            ),
            vec![
                "one".to_owned(),
                "two".to_owned(),
                "three".to_owned(),
                "four".to_owned(),
                "five".to_owned(),
                "six".to_owned()
            ]
        );
        assert_eq!(
            Solution::split_words_by_separator_fn(
                vec!["$easy$".to_owned(), "$problem$".to_owned()],
                '$'
            ),
            vec!["easy".to_owned(), "problem".to_owned()]
        );
        assert_eq!(
            Solution::split_words_by_separator_fn(vec!["|||".to_owned()], '|'),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_split_words_by_separator_command() {
        assert_eq!(
            Solution::split_words_by_separator_command(
                vec!["one.two.three".to_owned(), "four.five".to_owned(), "six".to_owned()],
                '.'
            ),
            vec![
                "one".to_owned(),
                "two".to_owned(),
                "three".to_owned(),
                "four".to_owned(),
                "five".to_owned(),
                "six".to_owned()
            ]
        );
        assert_eq!(
            Solution::split_words_by_separator_command(
                vec!["$easy$".to_owned(), "$problem$".to_owned()],
                '$'
            ),
            vec!["easy".to_owned(), "problem".to_owned()]
        );
        assert_eq!(
            Solution::split_words_by_separator_command(vec!["|||".to_owned()], '|'),
            Vec::<String>::new()
        );
    }
}
