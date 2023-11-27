//! 判断通过操作能否让字符串相等 II

use super::Solution;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut cnt1 = [[0; 26]; 2];
        let mut cnt2 = [[0; 26]; 2];

        for (i, c) in s1.chars().enumerate() {
            let idx = i % 2;
            cnt1[idx][(c as u8 - b'a') as usize] += 1;
        }

        for (i, c) in s2.chars().enumerate() {
            let idx = i % 2;
            cnt2[idx][(c as u8 - b'a') as usize] += 1;
        }

        cnt1 == cnt2
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_longest_equal_subarray() {
        assert_eq!(
            Solution::check_strings("abcdba".to_string(), "cabdab".to_string()),
            true
        );
        assert_eq!(
            Solution::check_strings("abe".to_string(), "bea".to_string()),
            false
        );
    }
}
