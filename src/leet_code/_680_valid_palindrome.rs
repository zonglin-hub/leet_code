use super::Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let valid = |s: &[u8]| s.iter().rev().take(s.len()).eq(s.iter().take(s.len()));
        let s = s.into_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                return valid(&s[i + 1..=j]) || valid(&s[i..=j - 1]);
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_valid_palindrome() {
        assert!(Solution::valid_palindrome(String::from("aba")));
        assert!(Solution::valid_palindrome(String::from("abca")));
        assert_eq!(Solution::valid_palindrome(String::from("abc")), false);
    }
}
