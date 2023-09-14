//! 比特位计数

use crate::types::base_type::Solution;

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
    use super::*;

    #[test]
    fn test_count_bits() {
        /*
            输入：n = 2
            输出：[0,1,1]
            解释：
            0 --> 0
            1 --> 1
            2 --> 10
        */
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);

        /*
            输入：n = 5
            输出：[0,1,1,2,1,2]
            解释：
            0 --> 0
            1 --> 1
            2 --> 10
            3 --> 11
            4 --> 100
            5 --> 101
        */
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
