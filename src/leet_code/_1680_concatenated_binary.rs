use crate::leet_code::Solution;

impl Solution {
    pub fn concatenated_binary_v1(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut ans = 0i64;
        for i in 1..=n as i64 {
            let bit_len = i.ilog2() + 1; // i 的二进制位数
            ans = (ans << bit_len | i as i64) % MOD;
        }

        ans as i32
    }
}

impl Solution {
    pub fn concatenated_binary_v2(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let max_bits = (n as u32).ilog2() + 1;
        let mut pow2 = vec![0i64; max_bits as usize + 1];
        pow2[0] = 1;
        for i in 1..=max_bits as usize {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
        }

        let mut ans = 0i64;
        for i in 1..=n {
            let bits = (i as u32).ilog2() + 1;
            ans = (ans * pow2[bits as usize] + i as i64) % MOD;
        }
        ans as i32
    }
}

impl Solution {
    const MOD: i64 = 1_000_000_007;

    /// 快速幂：计算 (x^exp) % MOD
    fn pow_mod(mut x: i64, mut exp: i64) -> i64 {
        let mut res = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * x) % Self::MOD;
            }
            x = (x * x) % Self::MOD;
            exp /= 2;
        }
        res
    }

    pub fn concatenated_binary(n: i32) -> i32 {
        let n = n as u64; // 转换为 u64 便于比较
        let mut ans = 0i64;
        let mut w = 1;

        loop {
            // 当前位数 w 的最小数字：2^(w-1)
            let l = 1u64 << (w - 1);
            if l > n {
                break;
            }
            // 当前位数 w 的最大数字：2^w - 1，但不能超过 n
            let r = ((1u64 << w) - 1).min(n);
            let m = (r - l + 1) as i64; // 本组数字个数
            let r_mod = (r % (Self::MOD as u64)) as i64; // 最大值在模意义下的值

            // 模 MOD 下的 2^w
            let q = Self::pow_mod(2, w as i64);
            // (2^w)^m = 2^(w*m)
            let pow_q = Self::pow_mod(q, m);

            // (q-1) 的逆元
            let inv_q1 = Self::pow_mod((q - 1 + Self::MOD) % Self::MOD, Self::MOD - 2);

            // ---- 计算本组的拼接值 s ----
            // term1 = r * (pow_q - 1) / (q - 1)
            let term1 = (r_mod * ((pow_q - 1 + Self::MOD) % Self::MOD)) % Self::MOD;
            let term1 = (term1 * inv_q1) % Self::MOD;

            // 分子部分：q - m * pow_q + (m-1) * pow_q * q
            let t1 = (q - (m * pow_q) % Self::MOD + Self::MOD) % Self::MOD;
            let t2 = ((m - 1) * pow_q) % Self::MOD;
            let t2 = (t2 * q) % Self::MOD;
            let numerator = (t1 + t2) % Self::MOD; // 即分子

            // term2 = numerator / (q-1)^2
            let term2 = (numerator * inv_q1) % Self::MOD;
            let term2 = (term2 * inv_q1) % Self::MOD;

            // 本组和 s = term1 - term2
            let s = (term1 - term2 + Self::MOD) % Self::MOD;

            // 更新答案：ans = ans * 2^(w*m) + s
            ans = (ans * pow_q + s) % Self::MOD;

            w += 1;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_concatenated_binary() {
        assert_eq!(Solution::concatenated_binary(1), 1);
        assert_eq!(Solution::concatenated_binary(3), 27);
        assert_eq!(Solution::concatenated_binary(12), 505379714);
    }

    #[test]
    fn test_concatenated_binary_v1() {
        assert_eq!(Solution::concatenated_binary_v1(1), 1);
        assert_eq!(Solution::concatenated_binary_v1(3), 27);
        assert_eq!(Solution::concatenated_binary_v1(12), 505379714);
    }

    #[test]
    fn test_concatenated_binary_v2() {
        assert_eq!(Solution::concatenated_binary_v2(1), 1);
        assert_eq!(Solution::concatenated_binary_v2(3), 27);
        assert_eq!(Solution::concatenated_binary_v2(12), 505379714);
    }
}
