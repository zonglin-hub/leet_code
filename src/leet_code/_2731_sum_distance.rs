//! 移动机器人

use super::Solution;

impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut a = vec![0; nums.len()];
        let s = s.as_bytes();
        for (i, &x) in nums.iter().enumerate() {
            let d = if s[i] == b'L' { -d } else { d };
            a[i] = x as i64 + d as i64;
        }
        a.sort();

        let mut ans = 0i64;
        let mut sum = 0i64;
        for (i, &x) in a.iter().enumerate() {
            ans = (ans + i as i64 * x - sum) % MOD;
            sum += x;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_color_the_array() {
        assert_eq!(Solution::sum_distance(vec![-2, 0, 2], "RLL".to_string(), 3), 8);
        assert_eq!(Solution::sum_distance(vec![1, 0], "RL".to_string(), 2), 5);
    }
}
