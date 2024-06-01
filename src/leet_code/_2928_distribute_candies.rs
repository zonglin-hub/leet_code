use super::Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut ans = 0;
        for i in 0..limit + 1 {
            for j in 0..limit + 1 {
                if i + j > n {
                    break;
                }
                if n - i - j <= limit {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::distribute_candies(5, 2), 3);
        assert_eq!(Solution::distribute_candies(3, 3), 10);
    }
}
