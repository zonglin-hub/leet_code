//! 有效的括号

use super::Solution;

impl Solution {
    pub fn is_valid_v1(s: String) -> bool {
        let n = s.len();
        s.into_bytes()
            .drain(..)
            .fold(Vec::with_capacity(n), |mut s, x| {
                // println!("s: {:?}, x: {}\n", s, x);
                // s: [], x: 40
                // s: [40], x: 123
                // s: [40, 123], x: 91
                // s: [40, 123, 91], x: 93
                // s: [40, 123], x: 125
                // s: [40], x: 41
                match (s.pop(), x) {
                    (Some(b'['), b']') | (Some(b'('), b')') | (Some(b'{'), b'}') => (),
                    (Some(a), b) => {
                        s.push(a);
                        s.push(b)
                    }
                    // 第一次遍历必须满足 b'(' | b'[' | b'{'
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
        assert_eq!(Solution::is_valid_v1(String::from("(]")), false);
    }

    #[test]
    fn test_vec_drain() {
        let mut v = vec![1, 2, 3];
        let u: Vec<_> = v.drain(1..).collect();
        assert_eq!(v, &[1]);
        assert_eq!(u, &[2, 3]);

        // 全范围清除 vector，就像 `clear()` 一样
        v.drain(..);
        assert_eq!(v, &[]);
    }

    #[test]
    fn test_fold() {
        let a = [1, 4, 3];

        // 数组所有元素的总和
        // acc 累加器
        // x 迭代器遍历的数据
        let sum = a.iter().fold(0, |acc, x| {
            println!("acc: {}", acc);
            println!("x: {}", x);
            acc + x
        });

        assert_eq!(sum, 8);
    }
}
