//! 有效的括号

use super::Solution;

impl Solution {
    pub fn is_valid_v1(s: String) -> bool {
        let n = s.len();
        s.into_bytes()
            .drain(..)
            .fold(Vec::with_capacity(n), |mut s, x| {
                match (s.pop(), x) {
                    (Some(b'['), b']') | (Some(b'('), b')') | (Some(b'{'), b'}') => (),
                    (Some(a), b) => {
                        s.push(a);
                        s.push(b)
                    }
                    (None, b) => {
                        s.push(b);
                    }
                };
                s
            })
            .is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_valid_v1() {
        assert!(Solution::is_valid_v1(String::from("()")));
        assert!(Solution::is_valid_v1(String::from("[]")));
        assert!(Solution::is_valid_v1(String::from("{}")));
        assert!(Solution::is_valid_v1(String::from("()[]{}")));
        assert!(Solution::is_valid_v1(String::from("({[]})")));
        assert!(!Solution::is_valid_v1(String::from("(]")));
    }
}
