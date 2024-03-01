use super::Solution;

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        fn valid_two(num1: i32, num2: i32) -> bool {
            num1 == num2
        }

        fn valid_three(num1: i32, num2: i32, num3: i32) -> bool {
            (num1 == num2 && num1 == num3) || (num1 + 1 == num2 && num2 + 1 == num3)
        }

        let n = nums.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            if i >= 2 {
                dp[i] = dp[i - 2] && valid_two(nums[i - 2], nums[i - 1]);
            }

            if i >= 3 {
                dp[i] = dp[i] || (dp[i - 3] && valid_three(nums[i - 3], nums[i - 2], nums[i - 1]));
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert!(Solution::valid_partition(vec![4, 4, 4, 5, 6]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 2]));
    }
}
