use super::Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        sentence.bytes().fold(67108863, |x, y| x & !(1 << (y - 97))) == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert!(Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_owned()));
        assert!(!Solution::check_if_pangram("leetcode".to_owned()));
    }
}
