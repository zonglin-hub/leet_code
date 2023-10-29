use super::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if s.trim().is_empty() {
            return 0;
        }
        s.split_whitespace().last().unwrap().len() as i32
    }
}
