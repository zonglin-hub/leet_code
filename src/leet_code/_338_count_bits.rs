//! 比特位计数

use super::Solution;

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

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_count_bits() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
