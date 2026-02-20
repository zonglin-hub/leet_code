use crate::leet_code::Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        // 预计算的质数位掩码（32位整数）
        // 对于 i32，二进制中1的个数可能为0~32，其中质数有：
        // 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31
        // 掩码的二进制表示中，第 k 位为 1 表示 k 是质数。
        const PRIME_BITS_MASK: i32 = 0xA28AC;

        // 辅助函数：判断数字二进制中1的个数是否为质数
        fn has_prime_ones_count(num: i32) -> bool {
            let ones_count: u32 = num.count_ones(); // 步骤1：计算1的个数
            let bit_flag = 1 << ones_count; // 步骤2：构造位标志
            (bit_flag & PRIME_BITS_MASK) != 0 // 步骤3：检查质数掩码
        }

        (left..=right).filter(|&num| has_prime_ones_count(num)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_count_prime_set_bits() {
        assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
        assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
    }
}
