use super::Solution;

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let n = nums.len() - 1;
        let mut sum = nums[n];

        // for i in (0..n).rev() {
        for &val in nums[..n].iter().rev() {
            sum = if val <= sum { val + sum } else { val }
        }

        sum as i64
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::max_array_value(vec![2, 3, 7, 9, 3]), 21);
        assert_eq!(Solution::max_array_value(vec![5, 3, 3]), 11);
    }
}
