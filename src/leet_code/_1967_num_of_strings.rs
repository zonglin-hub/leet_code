//！作为子字符串出现在单词中的字符串数目

use super::Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .into_iter()
            .filter(|s| word.contains(s))
            .count()
            .try_into()
            .expect("")
    }
}
