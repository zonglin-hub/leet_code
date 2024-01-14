use super::Solution;

impl Solution {
    /// 该函数是一个生成格雷码的函数。格雷码是一种二进制编码，它的特点是相邻的两个码之间只有一位不同。这个函数使用了一个简单的算法来生成格雷码。
    ///
    /// 首先，它使用`1 << n`将 0 扩展到 n 位。然后，它使用`map`函数将每个位进行操作。对于每个位，它将该位向右移动一位，然后与原始位进行异或操作。最后，它使用`collect`函数将结果收集到一个向量中。
    ///
    /// 这个算法的思想是，对于每个位，将其向右移动一位，然后与原始位进行异或操作，这样就可以得到下一个格雷码。因为格雷码的特点是相邻的两个码之间只有一位不同，所以这个算法可以生成所有的格雷码。
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..1 << n).map(|f| (f >> 1) ^ f).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_gray_code() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
        assert_eq!(Solution::gray_code(1), vec![0, 1]);
    }
}
