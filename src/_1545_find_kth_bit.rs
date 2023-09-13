// https://leetcode.cn/problems/find-kth-bit-in-nth-binary-string/

#![allow(unused)]
use crate::types::base_type::Solution;

impl Solution {
    /// 找出第 N 个二进制字符串中的第 K 位
    ///
    /// 给定整数 n 和 k，找出由 n 个字符组成的所有长度为 n 的二进制字符串中的第 k 位，并返回其对应的字符。
    /// 若 k=1，则该字符串的第一位始终为 0。如果 k 等于 2^(n-1)，则该字符串的中间字符始终为 1。
    /// 若 k 小于 2^(n-1)，则该字符串的第 k 位对应于第 n-1 个字符（即去掉开头字符后剩下的字符串）。因此递归调用 find_kth_bit 方法以查找该位所在的子字符串。
    /// 若 k 大于 2^(n-1)，则该字符串的第 k 位对应于第 n-1 个字符的相对位置。因此，我们需要在子字符串上执行类似的递归查找，但需要反转查找位置。如果从新计算出的字符为 0，则原字符应为 1，反之亦然。
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if k == 1 {
            return '0';
        }

        let mid = 1 << (n - 1);
        if k == mid {
            return '1';
        }

        if k < mid {
            return Self::find_kth_bit(n - 1, k);
        }

        if Self::find_kth_bit(n - 1, mid * 2 - k) == '0' {
            '1'
        } else {
            '0'
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert_eq!(1 << 0, 1);
        assert_eq!(1 << 1, 2);
        assert_eq!(1 << 2, 4);
        assert_eq!(1 << 3, 8);
        assert_eq!(1 << 4, 16);
    }
}
