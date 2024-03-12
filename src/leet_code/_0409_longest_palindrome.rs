use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn longest_palindrome_409(s: String) -> i32 {
        let mut map = HashMap::new();

        for c in s.chars() {
            map.insert(c, map.get(&c).unwrap_or(&0) + 1);
        }

        let mut odd = false;
        let mut res = 0;
        for v in map.values() {
            res += v;

            if v % 2 == 1 {
                odd = true;
                res -= 1;
            }
        }

        res + if odd { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::longest_palindrome_409("abccccdd".to_owned()), 7);
        assert_eq!(Solution::longest_palindrome_409("a".to_owned()), 1);
        assert_eq!(Solution::longest_palindrome_409("aaaaaccc".to_owned()), 7);
    }
}
