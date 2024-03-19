use std::cmp;

use super::Solution;

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let word = word.as_bytes();
        let len = word.len();

        let mut m = len + cmp::min(word[0] - b'a', b'a' + 26 - word[0]) as usize;

        for i in 1..len {
            let a = cmp::min(word[i], word[i - 1]);
            let b = cmp::max(word[i], word[i - 1]);
            m += cmp::min(b - a, a + 26 - b) as usize;
        }
        m as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::min_time_to_type("abc".to_owned()), 5);
        assert_eq!(Solution::min_time_to_type("bza".to_owned()), 7);
    }
}
