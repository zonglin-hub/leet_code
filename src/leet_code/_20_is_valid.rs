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
