use super::Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        #[inline]
        fn check(word1: &[u8], word2: &[u8]) -> bool {
            let len = word1.len();
            let mut diff_count = 0;

            for i in 0..len {
                if word1[i] != word2[i] {
                    diff_count += 1;
                    if diff_count > 1 {
                        return false;
                    }
                }
            }

            diff_count == 1
        }

        let mut word_list = word_list.into_iter().collect::<HashSet<_>>();
        if !word_list.contains(&end_word) {
            return 0;
        }
        word_list.remove(&begin_word);

        let mut min_count = 2;
        let mut queue = VecDeque::new();
        queue.push_back(begin_word);

        while !queue.is_empty() {
            let size = queue.len();

            for _ in 0..size {
                let cur_word = queue.pop_front().unwrap();
                let mut temp_word_list = HashSet::new();

                for word in word_list.into_iter() {
                    if check(cur_word.as_bytes(), word.as_bytes()) {
                        if word == end_word {
                            return min_count;
                        }
                        queue.push_back(word);
                    } else {
                        temp_word_list.insert(word);
                    }
                }

                word_list = temp_word_list;
            }

            min_count += 1;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_ladder_length() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ),
            5
        );
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string()
                ]
            ),
            0
        );
    }
}
