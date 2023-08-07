pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
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
            .is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let a = [1, 2, 3];
        let sum = a.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 6);
        assert!(Solution::is_valid(String::from("()[]{}")));
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}
