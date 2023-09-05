#![allow(unused)]
pub struct Solution;

impl Solution {
    /// 爬楼梯
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        (0..n).for_each(|_| {
            b += a;
            a = b - a;
        });
        a
    }

    pub fn climb_stairs_1(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            _ => Self::climb_stairs_1(n - 1) + Self::climb_stairs_1(n - 2),
        }
    }

    pub fn climb_stairs_2(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;
        (2..=n).for_each(|i| dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize]);
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
