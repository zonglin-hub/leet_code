use super::Solution;

impl Solution {
    pub fn is_palindrome_125(s: String) -> bool {
        let mut s = s.chars().filter(|c| c.is_alphanumeric());

        while let (Some(a), Some(b)) = (s.next(), s.next_back()) {
            if !a.eq_ignore_ascii_case(&b) {
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
    fn test_is_palindrome() {
        assert!(!Solution::is_palindrome_125("race a car".to_string()));
        assert!(Solution::is_palindrome_125(" ".to_string()));
        assert!(Solution::is_palindrome_125("A man, a plan, a canal: Panama".to_string()));
    }
}
