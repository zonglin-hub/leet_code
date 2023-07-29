#![allow(unused)]

pub struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/valid-parentheses/
    ///
    /// 有效的括号
    pub fn is_valid(mut s: String) -> bool {
        s.into_bytes()
            .drain(..)
            .fold(Vec::with_capacity(25), |mut s, x| {
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
            .len()
            == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold() {
        let a = [1, 2, 3];
        let sum = a.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_is_valid() {
        assert!(Solution::is_valid(String::from("()[]{}")));
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}
