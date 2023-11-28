//! 按奇偶性交换后的最大数字
//!

use super::Solution;

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut s = num.to_string().chars().collect::<Vec<char>>();
        let n = s.len();
        for i in 0..n - 1 {
            for j in i + 1..n {
                if s[i] < s[j] && s[i] as u8 % 2 == s[j] as u8 % 2 {
                    s.swap(i, j);
                }
            }
        }
        s.into_iter().collect::<String>().parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_largest_integer() {
        assert_eq!(Solution::largest_integer(1234), 3412);
        assert_eq!(Solution::largest_integer(65875), 87655);
    }
}
