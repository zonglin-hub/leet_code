use super::Solution;

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let n = std::cmp::min(s1.len(), std::cmp::min(s2.len(), s3.len()));
        let mut i = 0;

        while i < n && s2.as_bytes()[i] == s1.as_bytes()[i] && s3.as_bytes()[i] == s1.as_bytes()[i]
        {
            i += 1;
        }

        if i == 0 {
            -1
        } else {
            (s1.len() + s2.len() + s3.len() - i * 3) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_minimum_operations() {
        assert_eq!(
            Solution::find_minimum_operations("abc".to_owned(), "abb".to_owned(), "ab".to_owned()),
            2
        );
        assert_eq!(
            Solution::find_minimum_operations("dac".to_owned(), "bac".to_owned(), "cac".to_owned()),
            -1
        );
    }
}
