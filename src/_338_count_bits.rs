//! 比特位计数

use crate::Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; (n + 1) as usize];
        for i in 1..=n as usize {
            // 注意，这里是 i >> 1，而不是 i / 2
            res[i] = res[i >> 1] + (i & 1) as i32;
        }
        res
    }
}
