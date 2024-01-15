use super::Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut res = 0_u8;
        s.bytes().for_each(|x| res ^= x);
        t.bytes().for_each(|x| res ^= x);
        res.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_the_difference() {
        assert_eq!(Solution::find_the_difference("abcd".to_owned(), "abcde".to_owned()), 'e');
        assert_eq!(Solution::find_the_difference("".to_owned(), "y".to_owned()), 'y');
    }
}
