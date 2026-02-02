use super::Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut res = 1;
        let mut cur = 1;

        for i in 1..s.len() {
            if s.as_bytes()[i] == s.as_bytes()[i - 1] + 1 {
                cur += 1;
            } else {
                cur = 1;
            }
            res = i32::max(res, cur);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_longest_continuous_substring() {
        assert_eq!(Solution::longest_continuous_substring("abacaba".to_string()), 2);
        assert_eq!(Solution::longest_continuous_substring("abcde".to_string()), 5);
    }
}
