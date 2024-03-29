use super::Solution;

impl Solution {
    pub fn can_be_equal_v2(s1: String, s2: String) -> bool {
        let mut cnt1 = vec![vec![0; 26]; 2];
        let mut cnt2 = vec![vec![0; 26]; 2];

        for i in 0..s1.len() {
            cnt1[i % 2][s1.as_bytes()[i] as usize - 'a' as usize] += 1;
            cnt2[i % 2][s2.as_bytes()[i] as usize - 'a' as usize] += 1;
        }
        cnt1 == cnt2
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_can_be_equal() {
        assert!(Solution::can_be_equal_v2("abcd".to_owned(), "cdab".to_owned()));
        assert!(!Solution::can_be_equal_v2("abcd".to_owned(), "dacb".to_owned()));
    }
}
