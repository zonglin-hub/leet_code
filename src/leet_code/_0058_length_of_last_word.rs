use super::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if s.trim().is_empty() {
            return 0;
        }
        s.split_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
        assert_eq!(Solution::length_of_last_word("luffy is still joyboy".to_string()), 6);
    }
}
