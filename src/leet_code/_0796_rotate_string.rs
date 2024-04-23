use super::Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && goal.repeat(2).contains(&s)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert!(Solution::rotate_string("abcde".to_owned(), "cdeab".to_owned()));
        assert!(!Solution::rotate_string("abcde".to_owned(), "abced".to_owned()));
    }
}
