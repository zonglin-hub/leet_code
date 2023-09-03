#![allow(unused)]
struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        // 使用fold函数，将0到n的值求和，并返回结果
        (0..n).fold((0, 1), |mut t, _| (t.1, t.0 + t.1)).0
    }

    pub fn fib_1(n: i32) -> i32 {
        // 如果n是0或1，则返回n，否则返回fib_1(n-2) + fib_1(n-1)
        match n {
            0 | 1 => n,
            _ => Self::fib_1(n - 2) + Self::fib_1(n - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        // F(2) = F(1) + F(0) = 1 + 0 = 1
        assert_eq!(Solution::fib(2), 1);
        // F(3) = F(2) + F(1) = 1 + 1 = 2
        assert_eq!(Solution::fib_1(3), 2);
        // F(4) = F(3) + F(2) = 2 + 1 = 3
        assert_eq!(Solution::fib(4), 3);
    }
}
